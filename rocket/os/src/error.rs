
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HalError {
    BusError(String),
    UnexpectedDevice,
    ConfigurationError(String),
    GpioError(String),
    ReadError(String),
    WriteError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    HalError(HalError),
    SensorNotReady,
    CommunicationError(String),
    InvalidData,
    ConfigurationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentError {
    DriverError(DriverError),
    LogicError(String),
    NotInitialized,
}

// Top-level error combining others
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RocketError {
    Hal(HalError),
    Driver(DriverError),
    Component(ComponentError),
    Kernel(String), // For simulated kernel errors
    Configuration(String),
}

impl fmt::Display for RocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocketError::Hal(e) => write!(f, "HAL Error: {:?}", e),
            RocketError::Driver(e) => write!(f, "Driver Error: {:?}", e),
            RocketError::Component(e) => write!(f, "Component Error: {:?}", e),
            RocketError::Kernel(s) => write!(f, "Kernel Error: {}", s),
            RocketError::Configuration(s) => write!(f, "Configuration Error: {}", s),
        }
    }
}

impl std::error::Error for RocketError {}

// Conversion implementations (optional but helpful)
impl From<HalError> for RocketError {
    fn from(e: HalError) -> Self {
        RocketError::Hal(e)
    }
}

impl From<DriverError> for RocketError {
    fn from(e: DriverError) -> Self {
        RocketError::Driver(e)
    }
}

impl From<ComponentError> for RocketError {
    fn from(e: ComponentError) -> Self {
        RocketError::Component(e)
    }
}

// Allow drivers to convert HAL errors easily
impl From<HalError> for DriverError {
    fn from(e: HalError) -> Self {
        DriverError::HalError(e)
    }
}

// Allow components to convert Driver errors easily
impl From<DriverError> for ComponentError {
    fn from(e: DriverError) -> Self {
        ComponentError::DriverError(e)
    }
}

pub type Result<T> = std::result::Result<T, RocketError>;
