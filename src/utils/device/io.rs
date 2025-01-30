/*!
  \file io.rs
  \brief A utility class for performing I/O operations on device files.

  The `IO` struct provides methods to:
  - Read the contents of a device file.
  - Append data to a device file.
  - Write data to a device file, overwriting its existing content.

  These methods are designed to simplify common file operations and provide
  detailed error messages in case of failures.

  \Author:      Logic.Cavalier
  \Date:        2025-01-05
  \Version:     1.0.0
  \License:     AGPLv3+
*/

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Each method provides detailed error messages when an operation fails,
/// such as when the file cannot be opened, read, or written to.
///
/// # Notes
///
/// - The `IO` struct does not maintain any internal state.
/// - All methods are static and can be called directly without creating
/// an instance of `IO`.
pub struct IO;

impl IO {
    /// Reads the content of a device file and returns it as a `String`.
    ///
    /// # Arguments
    ///
    /// * `device_file` - The path to the device file to read from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file content as a `String` if successful,
    /// or an error message as a `String` if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, read, or a line
    /// cannot be processed.
    pub fn read_from_device(device_file: &str) -> Result<String, String> {
        let mut read_value = String::new();
        let file_path = Path::new(device_file);

        // Open file for reading
        let file = File::open(file_path).map_err(|e| {
            format!(
                "[ ERROR ] Failed to read from device file {}: {}",
                device_file, e
            )
        })?;

        // Read the file line by line
        for line in io::BufReader::new(file).lines() {
            let line = line.map_err(|e| {
                format!(
                    "[ ERROR ] Failed to read line from device file {}: {}",
                    device_file, e
                )
            })?;
            read_value.push_str(&line);
            read_value.push('\n');
        }

        Ok(read_value)
    }

    /// Writes a value to a device file, overwriting any existing content.
    ///
    /// # Arguments
    ///
    /// * `device_file` - The path to the device file to write to.
    /// * `write_value` - The value to write to the device file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`true`) or an error message (`String`)
    /// if an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or the value cannot be written.
    pub fn write_to_device(device_file: &str, write_value: &str) -> Result<bool, String> {
        let file_path = Path::new(device_file);

        // Open file for writing (overwriting any existing content)
        let mut file = OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(false)
            .open(file_path)
            .map_err(|e| {
                format!(
                    "[ ERROR ] Failed to write to device file {}: {}",
                    device_file, e
                )
            })?;

        // Write the value to the file
        file.write_all(write_value.as_bytes()).map_err(|e| {
            format!(
                "[ ERROR ] Failed to write to device file {}: {}",
                device_file, e
            )
        })?;

        Ok(true)
    }
}
