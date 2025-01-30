/*!
  \file DefaultControlParams.rs
  \brief Defines default fan control parameters for speed adjustment.

  This module contains constants that define the default speed values for both
  step-based and soft-based fan control methods. These constants are used to
  control the fan speed by specifying the minimum, medium, and maximum speed values for
  each method. The fan speed can be adjusted according to the selected control method,
  either step-wise or smoothly.

  \Author:    Logic.Cavalier
  \Date:      2025-01-05
  \Version:   1.0.0
  \License:   AGPLv3+
*/

/// Minimum temperature for step-based CPU temperature control.
/// This represents the lowest CPU temperature threshold for step-based control.
pub const CPU_TEMPERATURE_STEP_MIN: f64 = 40.00;

/// Mid-range temperature for step-based CPU temperature control.
/// This represents the intermediate CPU temperature threshold for step-based control.
pub const CPU_TEMPERATURE_STEP_MID: f64 = 60.00;

/// Maximum temperature for step-based CPU temperature control.
/// This represents the highest CPU temperature threshold for step-based control.
pub const CPU_TEMPERATURE_STEP_MAX: f64 = 80.00;

/// Minimum temperature for soft-based CPU temperature control.
/// This represents the lowest CPU temperature threshold for soft control (smooth adjustments).
pub const CPU_TEMPERATURE_SOFT_MIN: f64 = 30.00;

/// Mid-range temperature for soft-based CPU temperature control.
/// This represents the intermediate CPU temperature threshold for soft control (smooth adjustments).
pub const CPU_TEMPERATURE_SOFT_MID: f64 = 60.00;

/// Maximum temperature for soft-based CPU temperature control.
/// This represents the highest CPU temperature threshold for soft control (smooth adjustments).
pub const CPU_TEMPERATURE_SOFT_MAX: f64 = 90.00;

/// Minimum step value percent for fan speed control (step-based).
/// This represents the lowest speed adjustment value percent when using step-based fan control.
pub const FAN_SPEED_STEP_MIN: f64 = 25.0;

/// Low-medium step value percent for fan speed control (step-based).
/// This is the intermediate low value percent used for fan speed adjustments.
pub const FAN_SPEED_STEP_LMD: f64 = 50.0;

/// High-medium step value percent for fan speed control (step-based).
/// This is the intermediate high value percent used for fan speed adjustments.
pub const FAN_SPEED_STEP_HMD: f64 = 75.0;

/// Maximum step value percent for fan speed control (step-based).
/// This represents the highest speed adjustment value percent when using step-based fan control.
pub const FAN_SPEED_STEP_MAX: f64 = 100.0;

/// Minimum soft value percent for fan speed control (soft-based).
/// This represents the lowest speed adjustment value percent for smooth fan control.
pub const FAN_SPEED_SOFT_MIN: f64 = 25.0;

/// Mid-range soft value percent for fan speed control (soft-based).
/// This value represents the medium fan speed in soft control mode.
pub const FAN_SPEED_SOFT_MID: f64 = 50.0;

/// Maximum soft value percent for fan speed control (soft-based).
/// This represents the highest speed adjustment value percent for smooth fan control.
pub const FAN_SPEED_SOFT_MAX: f64 = 100.0;

/// Off speed for fan control in percent.
/// This value represents the state where the fan is completely turned off (0% speed).
pub const FAN_CONTROL_SPEED_OFF: f64 = 0.0;

/// Minimum speed for fan control in percent.
/// This is the minimum speed that the fan can run at when turned on.
pub const FAN_CONTROL_SPEED_MIN: f64 = 25.0;

/// Maximum speed for fan control in percent.
/// This is the maximum speed that the fan can reach.
pub const FAN_CONTROL_SPEED_MAX: f64 = 100.0;
