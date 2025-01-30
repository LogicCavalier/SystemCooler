//!
//! \file handler.rs
//! \brief A signal handler for gracefully handling termination signals like
//! `SIGINT` (Ctrl+C) and `SIGTERM`.
//!
//! A signal handler for gracefully handling termination signals like
//! `SIGINT` (Ctrl+C) and `SIGTERM`.
//!
//! \Author:    Logic.Cavalier
//! \Date:      2025-01-05
//! \Version:   1.1.0
//! \License:   AGPLv3+
//!
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// A struct to handle termination signals such as `SIGINT` (Ctrl+C) and
/// `SIGTERM` (pkill command).
///
/// The `Terminator` listens for termination signals and allows checking
/// if a signal was received.
pub struct Terminator {
    received_signal: Arc<Mutex<Option<i32>>>,
}

impl Terminator {
    /// Creates a new instance of `Terminator`.
    ///
    /// This initializes the internal state to keep track of received signals.
    pub fn new() -> Self {
        Terminator {
            received_signal: Arc::new(Mutex::new(None)),
        }
    }

    /// Starts listening for termination signals in a separate thread.
    ///
    /// This function will listen for `SIGINT` (Ctrl+C) and
    /// `SIGTERM` (kill command).
    /// It runs indefinitely, blocking until the process is terminated
    /// or a signal is received.
    pub fn listen_signals(&self) {
        let received_signal = Arc::clone(&self.received_signal);

        thread::spawn(move || {
            // Register for SIGINT and SIGTERM signals
            let mut signals =
                Signals::new(&[SIGINT, SIGTERM]).expect("[ ERROR ] Unable to register signals");

            // Continuously listen for signals
            for sig in signals.forever() {
                let mut signal = received_signal.lock().unwrap();
                *signal = Some(sig); // Store the received signal

                // Map signal number to its name
                let signal_name = match sig {
                    SIGINT => "SIGINT (Ctrl+C)",
                    SIGTERM => "SIGTERM (kill command)",
                    _ => "Unknown Signal",
                };

                println!("[ INFO ] Received signal: {}", signal_name);

                // You can add additional logic here to handle the signal,
                // like clean up or other actions before exiting
            }
        });
    }

    /// Checks if a termination signal has been received.
    ///
    /// This function will return `Some(signal_number)` if a signal
    /// has been received, or `None` if no signal has been received yet.
    pub fn check_signals(&self) -> Option<i32> {
        let signal = self.received_signal.lock().unwrap();
        *signal
    }
}
