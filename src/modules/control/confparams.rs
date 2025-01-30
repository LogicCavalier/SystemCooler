/*!
  \file confparams.rs
  \brief Defines default configuration parameters for fan control and system monitoring.

  This module contains constants that specify the default values for various configuration settings.
  These settings are used by the Fan Controller Daemon and other related modules to control fan speed,
  monitor CPU temperature, and configure network parameters. The default parameters help ensure
  that the system operates with predefined settings if no user-specific configuration file is provided.

  \Author:    Logic.Cavalier
  \Date:      2025-01-05
  \Version:   1.0.0
  \License:   AGPLv3+
*/

/// Step control method for fan speed adjustment.
/// The 'Step' method adjusts the fan speed in discrete steps.
pub const STEP_CONTROL_METHOD: &str = "Step";

/// Soft control method for fan speed adjustment.
/// The 'Soft' method adjusts the fan speed smoothly without discrete steps.
pub const SOFT_CONTROL_METHOD: &str = "Soft";

/// Default control method for the fan controller.
/// This method is used when no configuration is provided. The default is set to 'Step' control method.
pub const DEFAULT_CONTROL_METHOD: &str = STEP_CONTROL_METHOD;
