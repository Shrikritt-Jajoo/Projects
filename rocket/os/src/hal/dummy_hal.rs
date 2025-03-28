// Simulation implementation of the HAL traits
use crate::hal::interface::*;
use crate::error::{HalError, HalResult};
use crate::config;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use rand::Rng;
use lazy_static::lazy_static;
use chrono::Utc;

// --- Simulated Hardware State ---
// Use Mutex for interior mutability needed for simulation state.
// lazy_static! makes it easy to have global state for this simulation.
// WARNING: Global mutable state is generally discouraged, but simplifies this example.
struct DummyHardwareState {
    gpio_pins: HashMap<u8, bool>, // Pin number -> state (true=high, false=low)
    i2c_devices: HashMap<u8, Vec<u8>>, // Device address -> Register data
    spi_devices: HashMap<u8, Vec<u8>>, // Bus ID -> Dummy data buffer
    last_delay: Instant,
}

impl DummyHardwareState {
    fn new() -> Self {
        let mut i2c_devices = HashMap::new();
        // Pre-populate some dummy I2C device data (e.g., for IMU)
        i2c_devices.insert(config::DUMMY_IMU_ADDR, vec![0u8; 14]); // 6 accel, 2 temp, 6 gyro

        DummyHardwareState {
            gpio_pins: HashMap::new(),
            i2c_devices,
            spi_devices: HashMap::new(),
            last_delay: Instant::now(),
        }
    }
}

lazy_static! {
    static ref HW_STATE: Mutex<DummyHardwareState> = Mutex::new(DummyHardwareState::new());
}

// --- Dummy Implementations ---

// -- GPIO --
#[derive(Debug, Clone)]
pub struct DummyPin {
    pin_id: u8,
}

impl OutputPin for DummyPin {
    fn set_high(&mut self) -> HalResult<()> {
        let mut state = HW_STATE.lock().unwrap();
        println!("[HAL] GPIO Pin {} -> HIGH", self.pin_id);
        state.gpio_pins.insert(self.pin_id, true);
        Ok(())
    }

    fn set_low(&mut self) -> HalResult<()> {
        let mut state = HW_STATE.lock().unwrap();
         println!("[HAL] GPIO Pin {} -> LOW", self.pin_id);
        state.gpio_pins.insert(self.pin_id, false);
        Ok(())
    }
}

impl InputPin for DummyPin {
    fn is_high(&self) -> HalResult<bool> {
        let state = HW_STATE.lock().unwrap();
        let pin_state = state.gpio_pins.get(&self.pin_id).cloned().unwrap_or(false); // Default low if not set
        // println!("[HAL] GPIO Pin {} Read -> {}", self.pin_id, if pin_state { "HIGH" } else { "LOW" });
        Ok(pin_state)
    }
}

// -- I2C --
#[derive(Debug, Clone)]
pub struct DummyI2c {
    bus_id: u8,
}

impl I2cBus for DummyI2c {
    fn write(&mut self, address: u8, bytes: &[u8]) -> HalResult<()> {
        let mut state = HW_STATE.lock().unwrap();
        println!("[HAL] I2C[{}] Write to 0x{:02X}: {:02X?}", self.bus_id, address, bytes);
        if let Some(device_data) = state.i2c_devices.get_mut(&address) {
            // Simulate writing to registers - very basic simulation
            // Assumes first byte is register address, rest is data
            if bytes.len() > 1 {
                let reg_addr = bytes[0] as usize;
                let data_to_write = &bytes[1..];
                if reg_addr + data_to_write.len() <= device_data.len() {
                    device_data[reg_addr..reg_addr + data_to_write.len()].copy_from_slice(data_to_write);
                } else {
                    println!("[HAL] I2C[{}] Write out of bounds to 0x{:02X}", self.bus_id, address);
                     // Don't return error, just log for simulation
                }
            }
            Ok(())
        } else {
            Err(HalError::UnexpectedDevice)
        }
    }

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> HalResult<()> {
        let mut state = HW_STATE.lock().unwrap();
        println!("[HAL] I2C[{}] Read from 0x{:02X} ({} bytes)", self.bus_id, address, buffer.len());
        if let Some(device_data) = state.i2c_devices.get_mut(&address) {
            // Simulate reading sensor data with some noise
            let mut rng = rand::thread_rng();
            let data_len = device_data.len().min(buffer.len());

            // Simulate reading from start of device memory for simplicity
             buffer[..data_len].copy_from_slice(&device_data[..data_len]);

             // Add some noise (example for IMU-like data)
            if address == config::DUMMY_IMU_ADDR && data_len >= 14 {
                 // Simulate noise on accelerometer/gyro data (bytes 0-5, 8-13)
                for i in (0..6).chain(8..14) {
                    if i < buffer.len() {
                        buffer[i] = buffer[i].wrapping_add(rng.gen_range(0..5)); // Add small random noise
                    }
                }
                // Simulate slight temperature drift (bytes 6-7)
                 if 7 < buffer.len() {
                      let temp_change: i8 = rng.gen_range(-1..2); // -1, 0, or 1
                      let temp_bytes = &mut buffer[6..8];
                      let current_temp = i16::from_be_bytes([temp_bytes[0], temp_bytes[1]]);
                      let new_temp = current_temp.wrapping_add(temp_change as i16);
                      let new_bytes = new_temp.to_be_bytes();
                      temp_bytes[0] = new_bytes[0];
                      temp_bytes[1] = new_bytes[1];
                 }
            }

            println!("[HAL] I2C[{}] Read data: {:02X?}", self.bus_id, &buffer[..data_len]);
            Ok(())
        } else {
            Err(HalError::UnexpectedDevice)
        }
    }

    fn write_read(&mut self, address: u8, bytes_to_write: &[u8], buffer_to_read: &mut [u8]) -> HalResult<()> {
        // Simulate as separate write then read for simplicity
        self.write(address, bytes_to_write)?;
        // Add small delay to simulate bus turnaround
        std::thread::sleep(Duration::from_micros(50));
        self.read(address, buffer_to_read)
    }
}

// -- SPI --
#[derive(Debug, Clone)]
pub struct DummySpi {
     bus_id: u8,
}

impl SpiBus for DummySpi {
    fn transfer<'w>(&mut self, buffer: &'w mut [u8]) -> HalResult<&'w [u8]> {
        let mut state = HW_STATE.lock().unwrap();
        println!("[HAL] SPI[{}] Transfer: {:02X?}", self.bus_id, buffer);
        // Simulate loopback or reading predefined data
        let device_data = state.spi_devices.entry(self.bus_id).or_insert_with(|| vec![0xFF; buffer.len()]); // Default to 0xFF if not present
        let read_len = buffer.len().min(device_data.len());
        let response = device_data[..read_len].to_vec(); // Copy data to send back
        // You could modify device_data based on `buffer` here if simulating write
        buffer[..read_len].copy_from_slice(&response);
        println!("[HAL] SPI[{}] Received: {:02X?}", self.bus_id, &buffer[..read_len]);
        Ok(&buffer[..read_len])
    }

    fn write(&mut self, bytes: &[u8]) -> HalResult<()> {
         let _state = HW_STATE.lock().unwrap();
         println!("[HAL] SPI[{}] Write: {:02X?}", self.bus_id, bytes);
         // Simulate writing to a device - maybe store the written bytes?
         // For now, just log it.
         Ok(())
    }
}

// -- Delay --
#[derive(Debug, Clone, Copy)]
pub struct DummyDelay;

impl DelayUs for DummyDelay {
    fn delay_us(&mut self, us: u32) {
        // Use std::thread::sleep for simulation
        // Add a small base delay to simulate overhead if desired
        let delay_duration = Duration::from_micros(us as u64);
        // println!("[HAL] Delaying for {} us", us); // Can be noisy
        std::thread::sleep(delay_duration);
        HW_STATE.lock().unwrap().last_delay = Instant::now(); // Update last delay time
    }
}

impl DelayMs for DummyDelay {
    fn delay_ms(&mut self, ms: u32) {
        let delay_duration = Duration::from_millis(ms as u64);
        // println!("[HAL] Delaying for {} ms", ms);
        std::thread::sleep(delay_duration);
        HW_STATE.lock().unwrap().last_delay = Instant::now(); // Update last delay time
    }
}


// --- Top Level Dummy HAL Provider ---
pub struct DummyHal;

impl FullHardwareAbstraction for DummyHal {
    type GpioPin = DummyPin;
    type I2cController = DummyI2c;
    type SpiController = DummySpi;
    type TimerDelay = DummyDelay;

    fn get_gpio_pin(&self, pin_id: u8) -> Option<Self::GpioPin> {
        println!("[HAL] Getting GPIO Pin {}", pin_id);
        Some(DummyPin { pin_id })
    }

    fn get_i2c_bus(&self, bus_id: u8) -> Option<Self::I2cController> {
        println!("[HAL] Getting I2C Bus {}", bus_id);
        // Only provide bus 0 in this simulation
        if bus_id == 0 { Some(DummyI2c { bus_id }) } else { None }
    }

     fn get_spi_bus(&self, bus_id: u8) -> Option<Self::SpiController> {
        println!("[HAL] Getting SPI Bus {}", bus_id);
         // Only provide bus 1 in this simulation
        if bus_id == config::DUMMY_RADIO_SPI_BUS { Some(DummySpi { bus_id }) } else { None }
    }

    fn get_delay_timer(&self) -> Self::TimerDelay {
         println!("[HAL] Getting Delay Timer");
         DummyDelay
    }
}

// Helper function to get the singleton instance
pub fn get_dummy_hal() -> DummyHal {
    DummyHal
}
