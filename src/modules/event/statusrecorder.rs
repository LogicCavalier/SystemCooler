//!
//! \file statusrecorder.rs
//! \brief Provides the `StatusRecorder` struct for logging status messages to the system log.
//!
//! Provides the `StatusRecorder` struct for logging status messages to the system log.
//!
//! This module defines the `StatusRecorder` struct, which facilitates logging messages
//! to the system log using the `syslog` crate. It supports different log levels,
//! including informational, warning, and error messages. The utility is particularly useful for
//! tracking events and errors in system daemons or applications, especially when working with `systemd`.
//!
//! \Author:    Logic.Cavalier
//! \Date:      2025-01-05
//! \Version:   1.0.0
//! \License:   AGPLv3+

use chrono;
use syslog::{Facility, Formatter3164};

/// A utility struct for recording status messages to the system log.
///
/// `StatusRecorder` provides methods for logging informational, warning, and error messages
/// to the system log. It uses the `syslog` crate and is compatible with `systemd`-based systems.
///
/// Each log message includes a timestamp and a predefined prefix for context.
/// This struct is especially useful for system-level daemons like `SystemCoolerDaemon`.
pub struct StatusRecorder;

impl StatusRecorder {
    /// Retrieves the current date and time as a formatted string.
    ///
    /// This function formats the current date and time into the format `yyyy-mm-dd hh:mm:ss`,
    /// which is suitable for inclusion in log messages.
    ///
    /// # Returns
    /// A `String` representing the current date and time.
    fn get_timestamp() -> String {
        let current_time = chrono::Local::now();
        current_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Creates and returns a `Logger` instance for logging to the system log.
    ///
    /// The logger is configured using `Formatter3164` with the `LOG_USER` facility,
    /// and the process name is set to "SystemCoolerDaemon".
    ///
    /// # Returns
    /// A `Logger` instance ready for use with logging methods like `info`, `warn`, or `err`.
    ///
    /// # Panics
    /// This function panics if it fails to create the logger instance.
    fn create_logger() -> syslog::Logger<syslog::LoggerBackend, Formatter3164> {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: "systemcoolerd".into(),
            pid: 0,
        };
        syslog::unix(formatter).unwrap()
    }

    /// Records an informational message to the system log.
    ///
    /// This method logs a message with the "info" priority level. The message is prefixed with the
    /// string `[ systemcoolerd ]` and includes a timestamp.
    ///
    /// # Arguments
    /// * `note` - A string slice containing the message to be logged.
    pub fn record_info(note: &str) {
        let timestamp = StatusRecorder::get_timestamp();
        let message = format!("[ systemcoolerd ] [ {} ] {}", timestamp, note);
        let mut logger = StatusRecorder::create_logger();
        logger.info(&message).unwrap();
    }

    /// Records a warning message to the system log.
    ///
    /// This method logs a message with the "warn" priority level. The message is prefixed with the
    /// string `[ systemcoolerd ]` and includes a timestamp.
    ///
    /// # Arguments
    /// * `note` - A string slice containing the warning message to be logged.
    pub fn record_warning(note: &str) {
        let timestamp = StatusRecorder::get_timestamp();
        let message = format!("[ systemcoolerd ] [ {} ] {}", timestamp, note);
        let mut logger = StatusRecorder::create_logger();
        logger.warning(&message).unwrap();
    }

    /// Records an error message to the system log.
    ///
    /// This method logs a message with the "err" priority level. The message is prefixed with the
    /// string `[ systemcoolerd ]` and includes a timestamp.
    ///
    /// # Arguments
    /// * `note` - A string slice containing the error message to be logged.
    pub fn record_error(note: &str) {
        let timestamp = StatusRecorder::get_timestamp();
        let message = format!("[ systemcoolerd ] [ {} ] {}", timestamp, note);
        let mut logger = StatusRecorder::create_logger();
        logger.err(&message).unwrap();
    }
}
