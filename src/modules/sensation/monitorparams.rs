/*!
  \file monitorparams.rs
  \brief Defines default CPU temperature parameters for monitoring.

  This module contains constants that define the default temperature thresholds for the CPU temperature monitoring system. These constants are used to determine the temperature range for different control methods (step-based and soft-based), as well as the minimum, maximum, and top temperature levels for safe operation. The temperature values help determine when to adjust the fan speed in response to the CPU's temperature.

  \Author: Logic.Cavalier
  \Date: 2025-01-05
  \Version: 1.0.0
  \License: AGPLv3+
*/

/// Minimum temperature for CPU monitoring.
/// This represents the lowest acceptable CPU temperature for monitoring.
pub const CPU_MONITOR_TEMPERATURE_MIN: f64 = 30.00;

/// Maximum temperature for CPU monitoring.
/// This represents the highest acceptable CPU temperature for monitoring.
pub const CPU_MONITOR_TEMPERATURE_MAX: f64 = 90.00;

/// Top temperature for CPU monitoring.
/// This represents the critical top temperature threshold for the CPU.
pub const CPU_MONITOR_TEMPERATURE_TOP: f64 = 100.00;
