// Wrapper around std::sync::Mutex to mimic RTOS mutex behavior (optional)
// In a real no_std RTOS, this would use critical sections or target-specific mutexes.
use std::sync::{Arc, Mutex as StdMutex, MutexGuard, LockResult};
use crate::error::{RocketError, Result};

#[derive(Debug)]
pub struct Mutex<T: ?Sized>(Arc<StdMutex<T>>);

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex(Arc::new(StdMutex::new(data)))
    }
}

// Implement Clone to allow sharing the Mutex handle
impl<T: ?Sized> Clone for Mutex<T> {
    fn clone(&self) -> Self {
        Mutex(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> Result<MutexGuard<'_, T>> {
        match self.0.lock() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => {
                // In a real RTOS, poisoning might require specific handling (e.g., system reset)
                eprintln!("WARN: Mutex poisoned! Attempting recovery.");
                // For simulation, try to recover by taking the lock anyway
                Ok(poisoned.into_inner())
            }
        }
    }
}

// Simple blocking delay - uses std::thread::sleep for simulation
pub fn sleep(duration: std::time::Duration) {
    std::thread::sleep(duration);
}

// Get current time - uses std::time::Instant for simulation
// A real RTOS would use a hardware timer.
pub fn get_time() -> std::time::Instant {
    std::time::Instant::now()
}

// Basic channel for inter-task communication (using std channels for simulation)
// In a real RTOS, this would be a bounded queue, possibly ISR-safe.
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

pub struct Channel<T> {
   tx: Sender<T>,
   rx: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Channel { tx, rx }
    }

    pub fn split(self) -> (ChannelSender<T>, ChannelReceiver<T>) {
        (ChannelSender { tx: self.tx }, ChannelReceiver { rx: self.rx })
    }
}

#[derive(Clone)]
pub struct ChannelSender<T> {
    tx: Sender<T>,
}

impl<T> ChannelSender<T> {
    pub fn send(&self, data: T) -> Result<()> {
        self.tx.send(data).map_err(|e| RocketError::Kernel(format!("Channel send error: {}", e)))
    }
}

pub struct ChannelReceiver<T> {
    rx: Receiver<T>,
}

impl<T> ChannelReceiver<T> {
    // Blocking receive
    pub fn recv(&self) -> Result<T> {
        self.rx.recv().map_err(|e| RocketError::Kernel(format!("Channel receive error: {}", e)))
    }

    // Non-blocking receive
    pub fn try_recv(&self) -> std::result::Result<Option<T>, RocketError> {
        match self.rx.try_recv() {
            Ok(data) => Ok(Some(data)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(RocketError::Kernel("Channel disconnected".to_string())),
        }
    }
}
