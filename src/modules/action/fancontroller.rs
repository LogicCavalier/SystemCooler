//! fancontroller.rs
//! Provides functionality for controlling a fan using PWM (Pulse Width Modulation).
//!
//! This module allows for controlling the speed of a fan by adjusting the PWM signal.
//! The fan speed is controlled by modifying the duty cycle of the PWM signal, which adjusts
//! the percentage of time the fan operates at full speed.
//!
//! Author:     Logic.Cavalier
//! Date:       2025-01-05
//! Version:    1.0.0
//! License:    AGPLv3+

use std::path::Path;

use crate::modules::action::confparams::DEFAULT_PWM_CHANNEL;
use crate::utils::device::io::IO;

const PWM_ENABLE_ACTIVE: &str = "1";
const PWM_ENABLE_DEACTIVE: &str = "0";
const PWM_PERIOD_STANDART: u64 = 20_000_000; // 20ms in nanoseconds
const PWM_ROOT: &str = "/sys/class/pwm/pwmchip0";
const PWM_ZONE: &str = "/pwm";
const PWM_EXPORT: &str = "/export";
const PWM_ENABLE: &str = "/enable";
const PWM_PERIOD: &str = "/period";
const PWM_DUTYCYCLE: &str = "/duty_cycle";

/// Enumeration for PWM status.
#[derive(Debug, PartialEq)]
pub enum PWMDeviceStatus {
    PwmdsEnabled,
    PwmdsDisabled,
}

/// Enumeration for PWM error codes.
#[derive(Debug, PartialEq)]
pub enum PWMSetErrorCodes {
    PwmsecNone,      // No error (not used in Result type)
    PwmsecExport,    // PWM export failure
    PwmsecEnable,    // PWM enable failure (not yet used in constructor)
    PwmsecPeriod,    // PWM period setting failure
    PwmsecDutyCycle, // PWM duty cycle setting failure (not yet used in constructor)
}

/// Struct for controlling the fan using PWM.
pub struct FanController {
    pwm_exported: bool,
    pwm_enabled: bool,
    pwm_channel: i32,
    pwm_dir_path: String,
    pwm_export_path: String,
    pwm_enable_path: String,
    pwm_period_path: String,
    pwm_dutycycle_path: String,
    pwm_device_status: PWMDeviceStatus,
    pub initialized: bool,
}

impl FanController {
    /// Constructor for creating a `FanController` instance.
    ///
    /// # Arguments
    /// * `pwm_channel` - The PWM channel to control the fan.
    ///
    /// This method now sets `initialized` to indicate success or failure.
    pub fn new(pwm_channel: i32) -> FanController {
        let fpwm_channel = if pwm_channel < DEFAULT_PWM_CHANNEL {
            DEFAULT_PWM_CHANNEL
        } else {
            pwm_channel
        };

        let fpwm_dir_path = format!("{}{}{}", PWM_ROOT, PWM_ZONE, fpwm_channel);
        let fpwm_export_path = format!("{}{}", PWM_ROOT, PWM_EXPORT);
        let fpwm_enable_path = format!("{}{}", fpwm_dir_path, PWM_ENABLE);
        let fpwm_period_path = format!("{}{}", fpwm_dir_path, PWM_PERIOD);
        let fpwm_dutycycle_path = format!("{}{}", fpwm_dir_path, PWM_DUTYCYCLE);

        let pwm_device_status = if Path::new(PWM_ROOT).exists() {
            PWMDeviceStatus::PwmdsEnabled
        } else {
            PWMDeviceStatus::PwmdsDisabled
        };

        if pwm_device_status == PWMDeviceStatus::PwmdsDisabled {
            eprintln!(
                "[ ERROR ] Failed to find PWM device on channel: {}",
                fpwm_channel
            );
            return FanController {
                pwm_exported: false,
                pwm_enabled: false,
                pwm_channel: fpwm_channel,
                pwm_dir_path: fpwm_dir_path,
                pwm_export_path: fpwm_export_path,
                pwm_enable_path: fpwm_enable_path,
                pwm_period_path: fpwm_period_path,
                pwm_dutycycle_path: fpwm_dutycycle_path,
                pwm_device_status,
                initialized: false, // Initialization failed
            };
        }

        if !Path::new(fpwm_dir_path.as_str()).exists() {
            // Export PWM channel
            if IO::write_to_device(&fpwm_export_path, &fpwm_channel.to_string()).is_err() {
                eprintln!("[ ERROR ] Failed to export PWM channel: {}", fpwm_channel);
                return FanController {
                    pwm_exported: false,
                    pwm_enabled: false,
                    pwm_channel: fpwm_channel,
                    pwm_dir_path: fpwm_dir_path,
                    pwm_export_path: fpwm_export_path,
                    pwm_enable_path: fpwm_enable_path,
                    pwm_period_path: fpwm_period_path,
                    pwm_dutycycle_path: fpwm_dutycycle_path,
                    pwm_device_status,
                    initialized: false, // Initialization failed
                };
            }
        }

        // Set PWM period (20ms for standard PWM)
        if IO::write_to_device(&fpwm_period_path, &PWM_PERIOD_STANDART.to_string()).is_err() {
            eprintln!(
                "[ ERROR ] Failed to set PWM period {} for channel: {}",
                PWM_PERIOD_STANDART, fpwm_channel
            );
            return FanController {
                pwm_exported: false,
                pwm_enabled: false,
                pwm_channel: fpwm_channel,
                pwm_dir_path: fpwm_dir_path,
                pwm_export_path: fpwm_export_path,
                pwm_enable_path: fpwm_enable_path,
                pwm_period_path: fpwm_period_path,
                pwm_dutycycle_path: fpwm_dutycycle_path,
                pwm_device_status,
                initialized: false, // Initialization failed
            };
        }

        FanController {
            pwm_exported: true,
            pwm_enabled: false,
            pwm_channel: fpwm_channel,
            pwm_dir_path: fpwm_dir_path,
            pwm_export_path: fpwm_export_path,
            pwm_enable_path: fpwm_enable_path,
            pwm_period_path: fpwm_period_path,
            pwm_dutycycle_path: fpwm_dutycycle_path,
            pwm_device_status,
            initialized: true, // Initialization succeeded
        }
    }

    /// Sets the PWM duty cycle, controlling the fan speed.
    ///
    /// # Arguments
    /// * `duty_cycle` - The duty cycle as a percentage (0.0 to 100.0).
    fn set_pwm_duty_cycle(&self, duty_cycle: f64) -> bool {
        let duty_cycle_ns = (duty_cycle * PWM_PERIOD_STANDART as f64) as u64;
        // let result = self.io.write(&self.fpwm_dutycycle_path, duty_cycle_ns.to_string());
        let result = IO::write_to_device(&self.pwm_dutycycle_path, &duty_cycle_ns.to_string());
        if let Ok(false) = result {
            eprintln!(
                "[ ERROR ] Failed to set PWM duty cycle on Channel: {}",
                self.pwm_channel
            );
        }
        match result {
            Ok(val) => val,  // If Ok, return the bool value
            Err(_) => false, // If Err, return false (or handle it differently)
        }
    }

    /// Enables or disables the specified PWM channel.
    ///
    /// # Arguments
    /// * `pwm_status` - `true` to enable, `false` to disable.
    fn set_pwm_enable(&mut self, pwm_status: bool) {
        let value = if pwm_status {
            PWM_ENABLE_ACTIVE
        } else {
            PWM_ENABLE_DEACTIVE
        };
        match IO::write_to_device(&self.pwm_enable_path, &value.to_string()) {
            Ok(true) => {
                self.pwm_enabled = pwm_status;
            }
            Ok(false) => {
                self.pwm_enabled = false;
            }
            Err(_) => {
                eprintln!(
                    "[ ERROR ] Failed to set PWM enable on Channel: {}",
                    self.pwm_channel
                );
            }
        }
    }

    /// Reads the current fan speed.
    ///
    /// # Returns
    /// `true` if the fan speed was successfully read, `false` if there was an error.
    pub fn read_fan_speed(&mut self) -> bool {
        if !self.pwm_enabled {
            return false; // PWM is not enabled
        }

        match IO::read_from_device(&self.pwm_dutycycle_path) {
            Ok(duty_cycle_str) => {
                let duty_cycle: f64 = duty_cycle_str
                    .parse()
                    .expect("[ Error ] Failed to parse string to f64");
                let fan_speed = (duty_cycle / PWM_PERIOD_STANDART as f64) * 100.0;
                println!("[ INFO ] Fan speed: {:.2}%", fan_speed);
                true // Successfully read fan speed
            }
            Err(_) => false, // Error reading duty cycle
        }
    }

    /// Writes the fan speed as a percentage (0.0 to 100.0).
    ///
    /// # Arguments
    /// * `fan_speed` - The desired fan speed as a percentage.
    ///
    /// # Returns
    /// `true` if the fan speed was successfully written, `false` if there was an error.
    pub fn write_fan_speed(&mut self, fan_speed: f64) -> bool {
        let duty_cycle = fan_speed / 100.0;
        if !self.set_pwm_duty_cycle(duty_cycle) {
            return false; // Failed to set PWM duty cycle
        }

        self.set_pwm_enable(true);
        if !self.pwm_enabled {
            return false; // Failed to enable PWM
        }

        true // Successfully set fan speed
    }
}
