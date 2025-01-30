//!
//! \file daemonizer.rs
//! \brief Daemonize Program with systemd Compatibility.
//!
//! This module provides functionality to daemonize a Rust program while being
//! compatible with `systemd`. Unlike typical daemonization, which detaches the
//! process from the terminal by forking and running in the background,
//! `systemd` is expected to start the process and manage its lifecycle.
//! Thus, this implementation avoids forking and includes logging to the system journal
//! instead of a log file.
//!
//! The `Daemonizer` struct here is designed to be compatible with systemd,
//! handling the proper initialization and managing the logging via `systemd`'s journal system.
//!
//! \Author:    Logic.Cavalier
//! \Date:      2025-01-05
//! \Version:   1.1.0
//! \License:   AGPLv3+

use std::io::{self};
use std::thread;
use std::time::Duration;

use crate::modules::action::fancontroller::{FanController, PWMSetErrorCodes};
use crate::modules::control::controlparams::FAN_CONTROL_SPEED_OFF;
use crate::modules::control::selectcontroller::SelectController;
use crate::modules::option::confhandler::ConfHandler;
use crate::modules::sensation::cpumonitor::CPUMonitor;
use crate::utils::signal::handler::Terminator;

/// A struct representing a daemonized program.
///
/// The `Daemonizer` struct handles the daemonization process of
/// a Rust program, which includes detaching from the terminal,
/// redirecting output to log files, and ensuring that the program
/// runs as a background service without terminal interaction.
pub struct Daemonizer;

impl Daemonizer {
    /// Daemonizes the current program
    ///
    /// # Errors
    ///
    /// This method may return an `io::Error` if the process fails to fork,
    /// create a new session, or redirect the output files.
    pub fn daemonize(&self, conf_file_path: &str) -> io::Result<()> {
        // Optionally, we could set up signal handling for SIGHUP to restart the daemon
        // Create a new instance of Terminator
        let terminator = Terminator::new();

        // Start listening for termination signals in a background thread
        terminator.listen_signals();

        // Attempt to create a ConfHandler instance from the configuration file
        let conf_handler = match ConfHandler::new(conf_file_path) {
            Ok(conf_handler) => {
                // Display the loaded configuration
                println!("[ INFO ] Control Method: {}", conf_handler.control_method());
                println!("[ INFO ] Thermal Zone: {}", conf_handler.thermal_zone());
                println!("[ INFO ] PWM Channel: {}", conf_handler.pwm_channel());
                println!("[ INFO ] Network Port: {}", conf_handler.network_port());
                conf_handler // Return the conf_handler out of this arm
            }
            Err(e) => {
                // Handle errors during configuration loading
                eprintln!("[ ERROR ] Failed to load configuration: {}", e);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to load configuration",
                ));
            }
        };

        // Initialize the FanController
        let mut fan_controller = FanController::new(conf_handler.pwm_channel());
        if fan_controller.initialized {
            println!("[ INFO ] Fan Controller initialized successfully.");
        } else {
            eprintln!("[ ERROR ] Failed to initialize Fan Controller.");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to initialize Fan Controller",
            ));
        };

        loop {
            // Check if a termination signal has been received
            if let Some(signal) = terminator.check_signals() {
                match signal {
                    // Handle different signals (e.g., SIGINT or SIGTERM)
                    2 => {
                        println!("[ INFO ] Received SIGINT (Ctrl+C) ...");
                        // ToDo: Power off the Fan
                        if fan_controller.write_fan_speed(FAN_CONTROL_SPEED_OFF) {
                            println!("[ INFO ] Fan is turned off ");
                        } else {
                            eprintln!("[ ERROR ] Failed to turn off the Fan.");
                        }
                        return Ok(()); // Normal termination
                    }
                    15 => {
                        println!("[ INFO ] Received SIGTERM (pkill command) ...");
                        // ToDo: Power off the Fan
                        return Ok(()); // Normal termination
                    }
                    _ => {
                        println!("[ WARN ] Received unknown signal: {}", signal);
                        // Continue running the daemon, as it's not a recognized termination signal
                    }
                }
            }

            // Initialize the CPUMonitor with configuration thermal zone (or 0 default for most CPUs)
            let mut cpu_monitor = CPUMonitor::new(conf_handler.thermal_zone());

            // Read the CPU temperature
            if cpu_monitor.read_cpu_temperature() {
                let cpu_temperature: f64 = cpu_monitor.cpu_temperature();
                println!("[ INFO ] CPU Temperature: {:.4} °C", cpu_temperature);

                // Use control and calculate fan speed according to configuration
                let mut selected_controller = SelectController::new();
                if selected_controller
                    .select_controller(conf_handler.control_method(), cpu_temperature)
                {
                    let fan_speed: f64 = selected_controller.get_fan_speed();

                    // Set fan speed and handle any errors
                    if fan_controller.write_fan_speed(fan_speed) {
                        println!("[ INFO ] Fan Speed: {:.4}%", fan_speed);
                    } else {
                        eprintln!("[ ERROR ] Failed to set fan speed to {:.4}%.", fan_speed);
                    }
                } else {
                    eprintln!(
                        "[ ERROR ] Failed to select controller based on {} control. Reattempt ...",
                        conf_handler.control_method()
                    );
                }
            } else {
                eprintln!(
                    "[ ERROR ] Failed to read CPU temperature on Thermal Zone: {}. Reattempt ...",
                    conf_handler.thermal_zone()
                );
            }

            // Simulate some work by sleeping for a short duration
            thread::sleep(Duration::from_secs(1));
        }
    }
}
