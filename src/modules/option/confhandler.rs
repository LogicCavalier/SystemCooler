/*!
  \file confhandler.rs
  \brief Contains the ConfHandler struct for parsing configuration parameters from an INI file.

  This module defines the `ConfHandler` struct, which is responsible for
  reading configuration parameters from an INI file and providing them through methods.
  The struct is initialized with a configuration file path and extracts several settings
  such as control methods, thermal zone, PWM channel, and network port.
  The configuration options are categorized under sections for control, hardware, and network.

  \Author:      Logic.Cavalier
  \Date:        2025-01-05
  \Version:     1.0.0
  \License:     AGPLv3+
*/

use configparser::ini::Ini;
use std::fs::File;
use std::io::{self, Read};

/// A struct for reading and storing configuration parameters from an INI file.
///
/// The `ConfHandler` struct is responsible for loading configuration settings from an INI file.
/// It provides access to parameters related to fan control methods, hardware settings,
/// and network port configurations. The configuration parameters include the control method
/// for fan management, thermal zone ID, PWM channel number, and network port for communication.
///
/// # Fields
/// - `control_method`: A string representing the fan control strategy (e.g., "Step" or "Soft").
/// - `thermal_zone`: The ID of the thermal zone for temperature monitoring.
/// - `pwm_channel`: The PWM channel used for fan control.
/// - `network_port`: The network port number for communication.
pub struct ConfHandler {
    control_method: String,
    thermal_zone: i32,
    pwm_channel: i32,
    network_port: i32,
}

impl ConfHandler {
    /// Getter for the `control_method` property.
    ///
    /// This method returns the current fan control method (e.g., "Step" or "Soft")
    /// read from the configuration file.
    ///
    /// # Return
    /// The control method string.
    pub fn control_method(&self) -> &str {
        &self.control_method
    }

    /// Getter for the `thermal_zone` property.
    ///
    /// This method returns the thermal zone ID that is used for temperature monitoring.
    /// The thermal zone corresponds to a specific hardware component (such as the CPU)
    /// where the temperature is monitored.
    ///
    /// # Return
    /// The thermal zone ID.
    pub fn thermal_zone(&self) -> i32 {
        self.thermal_zone
    }

    /// Getter for the `pwm_channel` property.
    ///
    /// This method returns the PWM channel number used for fan control.The PWM channel
    /// determines the fan speed based on the thermal data provided by the `thermal_zone`.
    ///
    /// # Return
    /// The PWM channel number.
    pub fn pwm_channel(&self) -> i32 {
        self.pwm_channel
    }

    /// Getter for the `network_port` property.
    ///
    /// This method returns the network port number configured in the INI file.
    /// The network port is used for communication over the network.
    ///
    /// # Return
    /// The network port number.
    pub fn network_port(&self) -> i32 {
        self.network_port
    }

    /// Constructor to load configuration parameters from an INI file.
    ///
    /// This method reads the configuration settings from an INI file located at
    /// the given `conf_file_path`. It loads values from different sections of the
    /// INI file (e.g., `[Control]`, `[Hardware]`, and `[Network]`) and populates the
    /// fields of the `ConfHandler` struct.
    ///
    /// If any configuration parameter is missing, default values are used.
    ///
    /// # Arguments
    /// - `conf_file_path`: The path to the configuration file.
    ///
    /// # Return
    /// A `Result` containing either the `ConfHandler` instance with the loaded
    /// configuration or an `io::Error` if an error occurred while reading the file
    /// or parsing the configuration.
    ///
    /// # Errors
    /// - If the INI file cannot be read or parsed, an `io::Error` will be returned.
    /// - If any configuration values cannot be parsed
    /// (e.g., non-numeric values for `thermal_zone`, `pwm_channel`, or `network_port`),
    ///   the default values will be used.
    pub fn new(conf_file_path: &str) -> Result<Self, io::Error> {
        let mut ini_file = File::open(conf_file_path)?;
        let mut contents = String::new();
        ini_file.read_to_string(&mut contents)?;

        // Load the configuration using configparser::ini::Ini
        let mut ini = Ini::new();

        // Convert the String error from ini.read to io::Error
        if let Err(e) = ini.read(contents) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }

        // Read values from the INI file with default fallbacks
        let control_method = ini
            .get("Control", "ControlMethod")
            .unwrap_or_else(|| String::from("DEFAULT_CONTROL_METHOD"));

        let thermal_zone: i32 = ini
            .get("Hardware", "ThermalZone")
            .unwrap_or_else(|| String::from("DEFAULT_THERMAL_ZONE"))
            .parse()
            .unwrap_or(0);

        let pwm_channel: i32 = ini
            .get("Hardware", "PWMChannel")
            .unwrap_or_else(|| String::from("DEFAULT_PWM_CHANNEL"))
            .parse()
            .unwrap_or(0);

        let network_port: i32 = ini
            .get("Network", "NetworkPort")
            .unwrap_or_else(|| String::from("DEFAULT_NETWORK_PORT"))
            .parse()
            .unwrap_or(0);

        // Return the ConfHandler with the loaded configuration
        Ok(ConfHandler {
            control_method,
            thermal_zone,
            pwm_channel,
            network_port,
        })
    }
}
