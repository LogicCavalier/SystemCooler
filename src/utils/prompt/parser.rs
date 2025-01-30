//! # parser.rs
//! Handles the command-line argument parsing for the System Cooler Daemon program.
//!
//! This module processes the command-line arguments passed to the
//! System Cooler Daemon daemon. It supports options for specifying a
//! configuration file and displaying the help message. The module parses the
//! `-c` or `--configuration` flag to determine the path to the configuration file
//! and checks for its existence. If an invalid argument or an error occurs, it raises an error.
//!
//! ## Author:      Logic.Cavalier
//! ## Date:        2025-01-05
//! ## Version:     1.0.0
//! ## License:     AGPLv3+

use std::env;
use std::fmt;
use std::path::Path;
use std::process::exit;
use std::string::String;

#[derive(Debug)]
/// Error raised when there is an issue with the command-line arguments.
pub struct PromptError {
    details: String,
}

impl PromptError {
    /// Creates a new `PromptError` with the given message.
    fn new(msg: &str) -> PromptError {
        PromptError {
            details: msg.to_string(),
        }
    }
}

/// Implement the `Display` trait for `PromptError`.
impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/// Implement the `std::error::Error` trait for `PromptError`.
impl std::error::Error for PromptError {}

/// The `Parser` struct is responsible for parsing command-line arguments.
pub struct Parser;

impl Parser {
    /// Parse the command-line arguments and return the path to the configuration file.
    /// If the configuration file is not provided or does not exist, it raises an error.
    pub fn parse_prompt() -> Result<String, PromptError> {
        let args: Vec<String> = env::args().collect();
        let mut conf_file_path = String::new(); // Default initialization as empty conf file path

        for i in 1..args.len() {
            match args[i].to_lowercase().as_str() {
                "-c" | "--configuration" => {
                    if i + 1 < args.len() {
                        conf_file_path = args[i + 1].clone();
                        break;
                    } else {
                        return Err(PromptError::new(
                            "Configuration file path not provided after -c/--configuration",
                        ));
                    }
                }
                "-v" | "--version" => {
                    Self::print_version();
                    exit(0);
                }
                "-h" | "--help" => {
                    Self::show_help();
                    exit(0);
                }
                _ => {
                    return Err(PromptError::new(&format!(
                        "[ERROR] Unknown option: {}",
                        args[i]
                    )))
                }
            }
        }

        Self::check_conf_file(&conf_file_path)?;

        Ok(conf_file_path)
    }

    /// Check if the configuration file path exists and is valid.
    /// If the file does not exist or is invalid, it raises an error.
    fn check_conf_file(conf_file_path: &str) -> Result<(), PromptError> {
        if conf_file_path.is_empty() {
            return Err(PromptError::new(
                "[ERROR] Configuration file path not provided. Use -c or --configuration.",
            ));
        }

        if !Path::new(conf_file_path).exists() {
            return Err(PromptError::new(&format!(
                "[ERROR] Configuration file not found: {}",
                conf_file_path
            )));
        }

        Ok(())
    }

    /// Print the current version of the Fan Controller Daemon.
    fn print_version() {
        println!();
        println!("{} {}", "System Cooler Daemon", "1.0.0");
        println!();
        println!("AGPLv3+ License");
        println!("Copyright 2025 Logic.Cavalier");
        println!();
    }

    /// Displays a usage guide for the command-line options.
    fn show_help() {
        println!(" Usage: systemcoolerd [options]");
        println!(" Options:");
        println!("   -c, --configuration <conf_file_path>   Specify the configuration file path.");
        println!("   -v, --version                          Print the version and exit.");
        println!("   -h, --help                             Show this help message and exit.");
    }
}
