//!
//! \file softcontroller.rs
//! \brief This module provides the `SoftController` struct, which implements a soft control
//! mechanism to calculate fan speed based on CPU temperature.
//!
//! This module contains the definition and implementation of the `SoftController` struct,
//! which calculates the fan speed using a linear relationship between the CPU temperature
//! and fan speed. The fan speed is calculated within predefined minimum and maximum values,
//! which are set by configuration constants.
//!
//! The soft control mechanism provides smooth adjustments in fan speed as the CPU temperature changes.
//!
//! \Author:    Logic.Cavalier
//! \Date:      2025-01-05
//! \Version:   1.0.0
//! \License:   AGPLv3+
//!

use crate::modules::control::controlparams::CPU_TEMPERATURE_SOFT_MAX;
use crate::modules::control::controlparams::CPU_TEMPERATURE_SOFT_MIN;

use crate::modules::control::controlparams::FAN_SPEED_SOFT_MAX;
use crate::modules::control::controlparams::FAN_SPEED_SOFT_MIN;

/// A struct that calculates fan speed using a linear relationship between
/// CPU temperature and fan speed, within predefined minimum and maximum values.
pub struct SoftController;

impl SoftController {
    /// Creates a new instance of `SoftController` with default or specified values.
    pub fn new() -> Self {
        SoftController
    }

    /// Calculates the fan speed based on the CPU temperature.
    ///
    /// This function computes the fan speed using a linear equation between
    /// the CPU temperature and the fan speed. It ensures the fan speed is within
    /// the predefined minimum and maximum values.
    ///
    /// # Arguments
    /// * `cpu_temperature` - The current CPU temperature in degrees Celsius.
    ///
    /// # Returns
    /// The calculated fan speed, clamped between the predefined minimum and maximum values (0.0 to 100.0).
    pub fn calculate_fan_speed(&self, cpu_temperature: f64) -> f64 {
        // Calculate the ratio for linear scaling between temperature and fan speed
        let speed_to_temperature_ratio = (FAN_SPEED_SOFT_MAX - FAN_SPEED_SOFT_MIN)
            / (CPU_TEMPERATURE_SOFT_MAX - CPU_TEMPERATURE_SOFT_MIN);

        // Calculate the fan speed
        let mut fan_speed = FAN_SPEED_SOFT_MIN
            + (cpu_temperature - CPU_TEMPERATURE_SOFT_MIN) * speed_to_temperature_ratio;

        // Clamp the result within the allowed range
        if fan_speed < FAN_SPEED_SOFT_MIN {
            fan_speed = FAN_SPEED_SOFT_MIN;
        } else if fan_speed > FAN_SPEED_SOFT_MAX {
            fan_speed = FAN_SPEED_SOFT_MAX;
        }

        fan_speed
    }
}
