use std::time::Duration;

// Simulation parameters
pub const SIM_TICK_RATE: Duration = Duration::from_millis(10); // Base tick for simulation delays

// Task loop rates (adjust as needed)
pub const NAV_LOOP_RATE: Duration = Duration::from_millis(50); // 20 Hz
pub const CONTROL_LOOP_RATE: Duration = Duration::from_millis(20); // 50 Hz
pub const TELEMETRY_LOOP_RATE: Duration = Duration::from_millis(200); // 5 Hz

// Simulated Hardware Configuration
pub const DUMMY_IMU_ADDR: u8 = 0x68;
pub const DUMMY_VALVE_PIN: u8 = 10; // Simulated GPIO pin number
pub const DUMMY_RADIO_SPI_BUS: u8 = 1; // Simulated SPI bus ID

// Component Configuration
pub const TARGET_APOGEE: f32 = 1000.0; // meters
