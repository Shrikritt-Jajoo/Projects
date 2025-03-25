// Mark modules public for visibility within the crate
pub mod error;
pub mod config;
pub mod kernel;
pub mod hal;
pub mod drivers;
pub mod components;

use crate::error::Result; // Use our top-level Result
use crate::kernel::{task, sync::{Mutex, sleep}}; // Use our kernel types
use crate::hal::dummy_hal::{self, DummyHal}; // Use the dummy HAL
use crate::hal::interface::{FullHardwareAbstraction, OutputPin}; // Import traits
use crate::drivers::{imu::Imu, valve::Valve, radio::Radio};
use crate::components::{
    navigation::{Navigation, SharedNavState},
    engine_control::{EngineControl, EngineCommand},
    telemetry::Telemetry,
};
use std::{sync::Arc, time::Duration}; // For Arc and Duration in simulation

fn main() -> Result<()> {
    println!("[Main] Starting Rocket OS Simulation...");

    // --- Initialization ---
    println!("[Main] Initializing HAL...");
    let board_hal = dummy_hal::get_dummy_hal(); // Get the singleton HAL instance

    // Get peripheral instances from the HAL
    let i2c_bus = board_hal.get_i2c_bus(0).ok_or(error::RocketError::Configuration("Failed to get I2C bus 0".into()))?;
    let spi_bus = board_hal.get_spi_bus(config::DUMMY_RADIO_SPI_BUS).ok_or(error::RocketError::Configuration("Failed to get SPI bus".into()))?;
    let delay_timer = board_hal.get_delay_timer();

    // Setup GPIO pins (using unwrap for simplicity in example, prefer proper error handling)
    let fuel_valve_pin = board_hal.get_gpio_pin(config::DUMMY_VALVE_PIN).unwrap();
    let oxidizer_valve_pin = board_hal.get_gpio_pin(config::DUMMY_VALVE_PIN + 1).unwrap(); // Use next pin
    let radio_cs_pin = board_hal.get_gpio_pin(20).unwrap(); // Example CS pin
    let radio_irq_pin = board_hal.get_gpio_pin(21).unwrap(); // Example IRQ pin (dummy input)


    println!("[Main] Initializing Drivers...");
    // Create driver instances, wrapped in Arc<Mutex> for sharing across tasks (threads)
    // Clone peripherals if they need to be used by multiple drivers independently (like Delay)
    let imu_driver = Arc::new(Mutex::new(Imu::new(i2c_bus.clone(), delay_timer.clone(), config::DUMMY_IMU_ADDR)?));
    let fuel_valve_driver = Arc::new(Mutex::new(Valve::new(fuel_valve_pin)?));
    let oxidizer_valve_driver = Arc::new(Mutex::new(Valve::new(oxidizer_valve_pin)?));
    let radio_driver = Arc::new(Mutex::new(Radio::new(spi_bus, radio_cs_pin, radio_irq_pin, delay_timer.clone())?));


    println!("[Main] Initializing Components...");
    // Create shared state objects
    let shared_nav_state = SharedNavState::new();

    // Create component instances
    let navigation_component = Navigation::new(imu_driver.clone(), shared_nav_state.clone());
    // Share valve drivers with EngineControl
    let engine_control_component = Arc::new(Mutex::new(EngineControl::new(
        fuel_valve_driver.clone(),
        oxidizer_valve_driver.clone(),
        shared_nav_state.clone(),
    )));
     // Share radio driver, nav state, engine state with Telemetry
    let telemetry_component = Telemetry::new(
        radio_driver.clone(),
        shared_nav_state.clone(),
        engine_control_component.clone(),
        imu_driver.clone(), // Pass IMU again (ugly, see telemetry.rs comment)
    );

    // Wrap mutable components in Mutex for task access
    let navigation_component = Arc::new(Mutex::new(navigation_component));
    let telemetry_component = Arc::new(Mutex::new(telemetry_component));


    // --- Task Definitions ---
    println!("[Main] Spawning Tasks...");

    // Navigation Task
    let nav_handle = {
        let nav_comp = Arc::clone(&navigation_component);
        task::spawn("Navigation", move || -> Result<()> {
            loop {
                let start_time = kernel::sync::get_time();
                { // Scope for mutex guard
                    let mut nav = nav_comp.lock()?;
                    if let Err(e) = nav.update() {
                        eprintln!("[Navigation Task] Error: {}", e);
                        // Decide on error handling: retry, log, enter safe mode?
                    }
                } // Mutex guard dropped here

                // Calculate sleep time to maintain loop rate
                let elapsed = start_time.elapsed();
                if elapsed < config::NAV_LOOP_RATE {
                    sleep(config::NAV_LOOP_RATE - elapsed);
                } else {
                     println!("[Navigation Task] WARN: Loop overrun!");
                }
            }
            // Ok(()) // Loop is infinite, Ok(()) is unreachable but needed for type signature
        })
    };

    // Control Task (Engine Control)
    let control_handle = {
        let engine_ctrl_comp = Arc::clone(&engine_control_component);
        task::spawn("Control", move || -> Result<()> {
             // --- Launch Sequence Simulation ---
            println!("[Control Task] Waiting 5 seconds before ignition attempt...");
            sleep(Duration::from_secs(5));
            {
                let mut engine_ctrl = engine_ctrl_comp.lock()?;
                engine_ctrl.execute_command(EngineCommand::Ignite)?;
            }
            sleep(Duration::from_secs(1)); // Give time for ignition state machine

             // --- Main Control Loop ---
            loop {
                let start_time = kernel::sync::get_time();
                {
                     let mut engine_ctrl = engine_ctrl_comp.lock()?;
                     if let Err(e) = engine_ctrl.update() {
                         eprintln!("[Control Task] Error: {}", e);
                     }
                } // Mutex guard dropped

                 // Sleep to maintain loop rate
                let elapsed = start_time.elapsed();
                 if elapsed < config::CONTROL_LOOP_RATE {
                     sleep(config::CONTROL_LOOP_RATE - elapsed);
                 } else {
                      println!("[Control Task] WARN: Loop overrun!");
                 }
            }
             // Ok(()) // Unreachable
        })
    };

    // Telemetry Task
    let telemetry_handle = {
         let telem_comp = Arc::clone(&telemetry_component);
        task::spawn("Telemetry", move || -> Result<()> {
            loop {
                let start_time = kernel::sync::get_time();
                {
                    let mut telem = telem_comp.lock()?;
                    if let Err(e) = telem.run_cycle() {
                        eprintln!("[Telemetry Task] Error: {}", e);
                    }
                } // Mutex guard dropped

                // Sleep to maintain loop rate
                 let elapsed = start_time.elapsed();
                 if elapsed < config::TELEMETRY_LOOP_RATE {
                     sleep(config::TELEMETRY_LOOP_RATE - elapsed);
                 } else {
                     println!("[Telemetry Task] WARN: Loop overrun!");
                 }
            }
             // Ok(()) // Unreachable
        })
    };


    // --- Start Scheduler (Simulation) ---
    // In this simulation, threads run automatically.
    // We join the handles to wait for them (though they run forever in this example).
    // A real RTOS `run_scheduler()` might never return.
    println!("[Main] All tasks spawned. Simulation running...");

    // Keep the main thread alive, or join task handles if they are expected to finish
    // Since our tasks loop forever, joining will block indefinitely.
    // For a real application, you'd have shutdown logic.
    // For simulation, we can let them run for a duration or until Ctrl+C.
    // We will join them here, which means you'll need Ctrl+C to stop.
    let _ = nav_handle.join();
    let _ = control_handle.join();
    let _ = telemetry_handle.join();

    println!("[Main] Simulation finished (tasks joined - likely interrupted).");
    Ok(())
}
