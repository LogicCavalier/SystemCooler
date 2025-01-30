// cpumonitor.rs
// Provides the `CPUMonitor` struct for reading and monitoring the CPU temperature.
//
// This module defines the `CPUMonitor` struct, which allows for reading the temperature from
// the system's thermal files. The temperature is returned in degrees Celsius. The thermal zone
// ID can be specified, and it defaults to the defined zone if the specified zone is invalid.
//
// The temperature is read from a file under the `/sys/class/thermal` directory in Linux, which
// contains temperature data for various components such as the CPU.
//
// Author: Logic.Cavalier
// Date: 2025-01-05
// Version: 1.0.0
// License: AGPLv3+

use std::{fs, io};

use crate::modules::sensation::confparams::DEFAULT_THERMAL_ZONE;
use crate::modules::sensation::monitorparams::CPU_MONITOR_TEMPERATURE_MIN;
use crate::utils::device::io::IO;

const THERMAL_ROOT: &str = "/sys/class/thermal";
const THERMAL_ZONE: &str = "/thermal_zone";
const THERMAL_TEMP: &str = "/temp";

/// CPUMonitor: A struct for monitoring the CPU temperature.
///
/// The struct reads the temperature from the system's thermal files and provides
/// an interface to retrieve the CPU temperature value.
///
/// It is initialized with a thermal zone ID and will default to the defined
/// default thermal zone if the specified zone is invalid.
pub struct CPUMonitor {
    thermal_zone: i32,
    thermal_file_path: String,
    cpu_temperature: f64,
}

impl CPUMonitor {
    /// Creates a new CPUMonitor instance with the specified thermal zone.
    ///
    /// # Arguments
    ///
    /// * `thermal_zone` - The thermal zone ID to monitor (typically 0 for CPU temperature).
    ///
    /// # Returns
    ///
    /// Returns a `CPUMonitor` instance.
    pub fn new(thermal_zone: i32) -> Self {
        let mut thermal_file_path = format!(
            "{}{}{}{}",
            THERMAL_ROOT, THERMAL_ZONE, thermal_zone, THERMAL_TEMP
        );

        let zone = if thermal_zone < DEFAULT_THERMAL_ZONE {
            thermal_file_path = format!(
                "{}{}{}{}",
                THERMAL_ROOT, THERMAL_ZONE, DEFAULT_THERMAL_ZONE, THERMAL_TEMP
            );
            DEFAULT_THERMAL_ZONE
        } else {
            thermal_zone
        };

        CPUMonitor {
            thermal_zone: zone,
            thermal_file_path,
            cpu_temperature: CPU_MONITOR_TEMPERATURE_MIN,
        }
    }

    /// Reads the CPU temperature from the thermal file and converts it to degrees Celsius.
    ///
    /// # Returns
    ///
    /// Returns `true` if the temperature was successfully read, otherwise `false`.
    pub fn read_cpu_temperature(&mut self) -> bool {
        match IO::read_from_device(&self.thermal_file_path) {
            Ok(content) => {
                match content.trim().parse::<f64>() {
                    Ok(temp) => {
                        self.cpu_temperature = temp / 1000.0; // Convert from millidegrees to degrees
                        true
                    }
                    Err(_) => {
                        eprintln!(
                            "[ ERROR ] Failed to parse temperature from device file: {}",
                            self.thermal_file_path
                        );
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("[ ERROR ] {}", e);
                false
            }
        }
    }

    /// Getter for the CPU temperature.
    ///
    /// # Returns
    ///
    /// Returns the CPU temperature in degrees Celsius.
    pub fn cpu_temperature(&self) -> f64 {
        self.cpu_temperature
    }
}
