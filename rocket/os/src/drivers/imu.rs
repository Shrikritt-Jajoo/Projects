use crate::hal::interface::{I2cBus, DelayMs};
use crate::error::{DriverError, DriverResult}; // Use specific Result type if defined, or top-level
use crate::error::Result as RocketResult; // Using top-level Result

const ACCEL_X_H: u8 = 0x3B; // Example register addresses

#[derive(Debug, Clone, Copy)]
pub struct ImuData {
    pub accel: [f32; 3], // m/s^2
    pub gyro: [f32; 3],  // rad/s
    pub temp: f32,       // degrees C
}

pub struct Imu<I2C, DELAY>
where
    I2C: I2cBus,
    DELAY: DelayMs,
{
    i2c: I2C,
    delay: DELAY,
    address: u8,
    // Add sensitivity/scaling factors based on datasheet/configuration
    accel_scale: f32,
    gyro_scale: f32,
}

impl<I2C, DELAY> Imu<I2C, DELAY>
where
    I2C: I2cBus,
    DELAY: DelayMs,
{
    pub fn new(i2c: I2C, delay: DELAY, address: u8) -> RocketResult<Self> {
        let mut imu = Self {
            i2c,
            delay,
            address,
            // Example scales (replace with actual values for a specific IMU like MPU6050/9250)
            accel_scale: 16384.0, // LSB/g for +/- 2g range
            gyro_scale: 131.0,    // LSB/deg/s for +/- 250 deg/s range
        };
        imu.init()?;
        Ok(imu)
    }

    fn init(&mut self) -> DriverResult<()> {
        println!("[Driver:IMU] Initializing IMU at address 0x{:02X}", self.address);
        self.delay.delay_ms(100); // Wait for sensor startup

        // Example: Wake up sensor (specific to IMU model)
        // self.write_register(0x6B, 0x00)?; // PWR_MGMT_1: Wake up

        // Example: Read WHO_AM_I register to verify connection
        let who_am_i = self.read_register(0x75)?; // WHO_AM_I register
        println!("[Driver:IMU] WHO_AM_I = 0x{:02X}", who_am_i);
        // Check if who_am_i matches expected value for the sensor
        // if who_am_i != 0x68 { /* return Err(DriverError::UnexpectedDevice) */ } // Example check

        // Configure accelerometer and gyroscope ranges/settings here...
        // self.write_register(0x1B, 0x00)?; // Gyro Config: +/- 250 deg/s
        // self.write_register(0x1C, 0x00)?; // Accel Config: +/- 2g

        self.delay.delay_ms(50);
        println!("[Driver:IMU] Initialization complete.");
        Ok(())
    }

    fn write_register(&mut self, register: u8, value: u8) -> DriverResult<()> {
        self.i2c.write(self.address, &[register, value])?;
        Ok(())
    }

    fn read_register(&mut self, register: u8) -> DriverResult<u8> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[register], &mut buffer)?;
        Ok(buffer[0])
    }

    pub fn read_data(&mut self) -> DriverResult<ImuData> {
        let mut buffer = [0u8; 14]; // 6 accel bytes, 2 temp bytes, 6 gyro bytes
        // Read all sensor data registers starting from ACCEL_X_H
        self.i2c.write_read(self.address, &[ACCEL_X_H], &mut buffer)?;

        // Parse data (assuming Big Endian format common in IMUs)
        let accel_raw = [
            i16::from_be_bytes([buffer[0], buffer[1]]),
            i16::from_be_bytes([buffer[2], buffer[3]]),
            i16::from_be_bytes([buffer[4], buffer[5]]),
        ];
        let temp_raw = i16::from_be_bytes([buffer[6], buffer[7]]);
        let gyro_raw = [
            i16::from_be_bytes([buffer[8], buffer[9]]),
            i16::from_be_bytes([buffer[10], buffer[11]]),
            i16::from_be_bytes([buffer[12], buffer[13]]),
        ];

        // Convert raw data to physical units
        let g = 9.81; // Standard gravity
        let accel = [
            accel_raw[0] as f32 / self.accel_scale * g,
            accel_raw[1] as f32 / self.accel_scale * g,
            accel_raw[2] as f32 / self.accel_scale * g,
        ];
        // Example temperature conversion (depends on sensor)
        let temp = (temp_raw as f32 / 340.0) + 36.53;
        let gyro = [
            (gyro_raw[0] as f32 / self.gyro_scale).to_radians(),
            (gyro_raw[1] as f32 / self.gyro_scale).to_radians(),
            (gyro_raw[2] as f32 / self.gyro_scale).to_radians(),
        ];

        Ok(ImuData { accel, gyro, temp })
    }
}

// Define DriverResult if you want more specific driver errors
type DriverResult<T> = std::result::Result<T, DriverError>;
