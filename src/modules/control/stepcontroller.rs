//!
//! \file stepcontroller.rs
//! \brief Contains the `StepController` struct, which implements a step-based
//! control mechanism to calculate fan speed based on CPU temperature thresholds.
//!
//! This module defines the `StepController` struct, which calculates the fan speed
//! in discrete steps based on CPU temperature. The fan speed changes according
//! to predefined temperature ranges, providing a simple yet efficient fan control
//! system for cooling purposes. The step-based approach adjusts fan speed at
//! distinct thresholds, ensuring that the cooling mechanism is responsive but not
//! overly granular.
//!
//! \Author:    Logic.Cavalier
//! \Date:      2025-01-05
//! \Version:   1.0.0
//! \License:   AGPLv3+
//!

use crate::modules::control::controlparams::CPU_TEMPERATURE_STEP_MAX;
use crate::modules::control::controlparams::CPU_TEMPERATURE_STEP_MID;
use crate::modules::control::controlparams::CPU_TEMPERATURE_STEP_MIN;

use crate::modules::control::controlparams::FAN_SPEED_STEP_HMD;
use crate::modules::control::controlparams::FAN_SPEED_STEP_LMD;
use crate::modules::control::controlparams::FAN_SPEED_STEP_MAX;
use crate::modules::control::controlparams::FAN_SPEED_STEP_MIN;

/// A struct that calculates fan speed using step-based thresholds.
/// Fan speed changes in discrete steps as the CPU temperature crosses
/// predefined ranges.
pub struct StepController;

impl StepController {
    /// Creates a new instance of `StepController` with default or specified values.
    pub fn new() -> Self {
        StepController
    }

    /// Calculates the fan speed based on the CPU temperature.
    ///
    /// This method uses predefined temperature thresholds to adjust the fan
    /// speed in discrete steps. The fan speed is calculated based on which
    /// temperature range the current CPU temperature falls into.
    ///
    /// # Arguments
    /// * `cpu_temperature` - The current CPU temperature in degrees Celsius.
    ///
    /// # Returns
    /// The calculated fan speed as a percentage, ranging from 0.0 to 100.0.
    pub fn calculate_fan_speed(&self, cpu_temperature: f64) -> f64 {
        // Determine fan speed based on CPU temperature thresholds
        if cpu_temperature < CPU_TEMPERATURE_STEP_MIN {
            FAN_SPEED_STEP_MIN
        } else if cpu_temperature < CPU_TEMPERATURE_STEP_MID {
            FAN_SPEED_STEP_LMD
        } else if cpu_temperature < CPU_TEMPERATURE_STEP_MAX {
            FAN_SPEED_STEP_HMD
        } else {
            FAN_SPEED_STEP_MAX
        }
    }
}
