/*!
  \file DefaultConfParams.rs
  \brief Defines default configuration parameters for fan control and system monitoring.

  This module contains constants that specify the default values for various configuration settings. These settings are used by the FanControllerDaemon and other related modules to control fan speed, monitor CPU temperature, and configure network parameters. The default parameters help ensure that the system operates with predefined settings if no user-specific configuration file is provided.

  \Author:    Logic.Cavalier
  \Date:      2025-01-05
  \Version:   1.0.0
  \License:   AGPLv3+
*/

/// Default network port for communication.
/// This is set to 8688 by default, which is used for network communication.
pub const DEFAULT_NETWORK_PORT: i32 = 8688;
