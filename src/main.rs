/*!
  \file main.rs
  \brief This program runs the System Cooler Daemon.

  This Daemon Program is designed to monitor the system's CPU temperature
  and adjust the fan speed accordingly. It reads the configuration file,
  sets up necessary signal handlers, and enters the main daemon loop.
  The daemon continuously monitors the CPU temperature, adjusts the fan speed,
  and manages its status based on configuration parameters.

  \Author:      Logic.Cavalier
  \Date:        2025-01-05
  \Version:     1.0.0
  \License:     AGPLv3+
*/

mod modules;
mod utils;

use crate::utils::daemon::daemonizer;
use crate::utils::prompt::parser::Parser;

fn main() {
    // Call `Parser::parse_prompt` and handle its result
    let conf_file_path = match Parser::parse_prompt() {
        Ok(path) => {
            println!("[ INFO ] Configuration file path: {}", path);
            path // Use the configuration file path
        }
        Err(e) => {
            eprintln!("{}", e); // Print the error if something goes wrong
            std::process::exit(1); // Exit if there's an error
        }
    };

    // Create an instance of the Daemonizer.
    let daemonizer = daemonizer::Daemonizer;

    // Attempt to daemonize the process.
    match daemonizer.daemonize(conf_file_path.as_str()) {
        Ok(()) => {
            println!("[ INFO ] Daemon started successfully.");
        }
        Err(err) => {
            eprintln!("[ ERROR ] Failed to start the daemon: ({})", err);
            std::process::exit(1);
        }
    }

    // Log when the daemon is exiting.
    println!("[ INFO ] Exit Fan Controller Daemon...");
}
