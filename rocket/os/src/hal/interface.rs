// Defines the Hardware Abstraction Layer traits
// These describe *what* hardware can do, not *how*.
use crate::error::HalError;
type HalResult<T> = std::result::Result<T, HalError>;

// --- Digital I/O ---
pub trait OutputPin {
    fn set_high(&mut self) -> HalResult<()>;
    fn set_low(&mut self) -> HalResult<()>;
    fn set_state(&mut self, state: bool) -> HalResult<()> {
        if state { self.set_high() } else { self.set_low() }
    }
}

pub trait InputPin {
    fn is_high(&self) -> HalResult<bool>;
    fn is_low(&self) -> HalResult<bool> {
        self.is_high().map(|s| !s)
    }
}

// --- Communication Buses ---
pub trait I2cBus {
    // Simplified: write and read methods
    fn write(&mut self, address: u8, bytes: &[u8]) -> HalResult<()>;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> HalResult<()>;
    fn write_read(&mut self, address: u8, bytes_to_write: &[u8], buffer_to_read: &mut [u8]) -> HalResult<()>;
}

pub trait SpiBus {
    // Simplified: transfer method (simultaneous write/read)
    fn transfer<'w>(&mut self, buffer: &'w mut [u8]) -> HalResult<&'w [u8]>;
    fn write(&mut self, bytes: &[u8]) -> HalResult<()>;
}

// --- Timers ---
pub trait DelayUs {
    fn delay_us(&mut self, us: u32);
}

pub trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

// Combine delay traits
pub trait Delay: DelayUs + DelayMs {}
impl<T: DelayUs + DelayMs> Delay for T {} // Blanket implementation

// --- Analog ---
pub trait Adc<WORD> {
     // Simplified: read a single channel
    type Error;
    fn read(&mut self, channel: u8) -> std::result::Result<WORD, Self::Error>; // Use associated error type
}

// Add other traits as needed (UART, PWM, etc.)

// Marker trait for a complete HAL implementation for a board/chip
pub trait FullHardwareAbstraction {
    type GpioPin: OutputPin + InputPin; // Example: A pin can be both
    type I2cController: I2cBus;
    type SpiController: SpiBus;
    type TimerDelay: Delay;
    // Add other peripheral types here...

    // Methods to get instances of peripherals
    fn get_gpio_pin(&self, pin_id: u8) -> Option<Self::GpioPin>;
    fn get_i2c_bus(&self, bus_id: u8) -> Option<Self::I2cController>;
    fn get_spi_bus(&self, bus_id: u8) -> Option<Self::SpiController>;
    fn get_delay_timer(&self) -> Self::TimerDelay;
    // ...
}
