use std::thread::{self, JoinHandle};
use crate::error::Result; // Using our top-level Result

// Represents a running task (thread in this simulation)
pub struct TaskHandle(JoinHandle<Result<()>>); // Store join handle

// Spawns a new task (thread)
// The task function `f` should return a `Result<()>`
pub fn spawn<F>(name: &'static str, f: F) -> TaskHandle
where
    F: FnOnce() -> Result<()> + Send + 'static,
{
    let builder = thread::Builder::new().name(name.to_string());
    let handle = builder.spawn(f).expect("Failed to spawn simulated task (thread)");
    println!("[Kernel] Spawned task: {}", name);
    TaskHandle(handle)
}

// Waits for a task to complete (joins the thread)
// In a real RTOS, you might wait on a task handle or event flag.
impl TaskHandle {
    pub fn join(self) -> Result<()> {
        match self.0.join() {
            Ok(task_result) => task_result, // Propagate the task's own Result
            Err(e) => {
                // This error means the thread panicked
                eprintln!("FATAL: Task panicked: {:?}", e);
                Err(crate::error::RocketError::Kernel("Task panicked".to_string()))
            }
        }
    }
}

// Basic scheduler loop (simulation - just keeps main thread alive)
// A real RTOS scheduler manages task states, priorities, and context switching.
pub fn run_scheduler() {
    println!("[Kernel] Scheduler running (simulation - main thread waits).");
    // In this simulation, tasks run independently as threads.
    // The main thread could loop indefinitely, perform health checks,
    // or wait for a shutdown signal. Here, we just park it.
    // Joining handles happens in main.rs after this conceptually "returns".
    loop {
        // Prevent this thread from consuming 100% CPU
        std::thread::park();
        // In a real system, this might be unparked by an interrupt or shutdown signal
    }
    // println!("[Kernel] Scheduler stopped."); // Unreachable in this simple loop
}
