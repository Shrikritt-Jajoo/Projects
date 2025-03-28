use crate::hal::interface::OutputPin;
use crate::error::Result as RocketResult;

pub struct Valve<Pin: OutputPin> {
    pin: Pin,
    is_open: bool,
}

impl<Pin: OutputPin> Valve<Pin> {
    pub fn new(mut pin: Pin) -> RocketResult<Self> {
        println!("[Driver:Valve] Initializing valve.");
        // Ensure valve starts closed
        pin.set_low()?; // Assuming LOW means closed
        Ok(Self { pin, is_open: false })
    }

    pub fn open(&mut self) -> RocketResult<()> {
        if !self.is_open {
            println!("[Driver:Valve] Opening valve.");
            self.pin.set_high()?; // Assuming HIGH means open
            self.is_open = true;
        }
        Ok(())
    }

    pub fn close(&mut self) -> RocketResult<()> {
        if self.is_open {
            println!("[Driver:Valve] Closing valve.");
            self.pin.set_low()?; // Assuming LOW means closed
            self.is_open = false;
        }
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}
