/*!
  \file infotransmitter.rs
  \brief Contains the `InfoTransmitter` struct for transmitting information over a
  network using a TCP server socket.

  This module contains

  \Author:    Logic.Cavalier
  \Date:      2025-01-05
  \Version:   1.0.0
  \License:   AGPLv3+
*/

use std::io::Write;
use std::net::TcpListener;
// use std::time::Duration;

const MAX_BUFFER_LEN: usize = 512;
const TIMEOUT_SEC: u64 = 1;
const PORT: u16 = 8464;

pub struct InfoTransmitter {
    server_socket: TcpListener,
}

impl InfoTransmitter {
    //! Constructor to initialize the server socket, bind it to a port, and start listening for incoming connections.
    //! # Errors
    //! If socket creation, binding, or listening fails, it will return an error.
    pub fn new() -> Result<Self, String> {
        // Set up the server socket.
        let server_socket =
            TcpListener::bind(format!("0.0.0.0:{}", PORT)).map_err(|_| "Failed to bind socket.")?;

        // Set the socket to allow for a timeout.
        server_socket
            .set_nonblocking(true)
            .map_err(|_| "[ Error ] Failed to set socket to non-blocking.")?;

        Ok(InfoTransmitter { server_socket })
    }

    /// Destructor to close the server socket and release resources.
    fn close(&self) {
        // TcpListener does not need explicit closing, it's dropped when the struct goes out of scope.
    }

    /// Handles an incoming request by accepting a connection from a client and sending the specified information.
    /// # Arguments
    /// * `info` - The information to transmit to the client.
    pub fn handle_request(&self, info: &str) {
        // Use select-like functionality by checking if there are incoming connections with a timeout.
        // let timeout = Duration::new(TIMEOUT_SEC, 0);
        let result = self
            .server_socket
            .accept()
            .map_err(|_| "No client connected.");

        match result {
            Ok((mut client_socket, _)) => {
                // Send the provided information to the client.
                if let Err(_) = client_socket.write_all(info.as_bytes()) {
                    eprintln!("[ Error ] Error sending data to client.");
                }
            }
            Err(_) => {
                // Timeout or error occurred, exit the function.
                return;
            }
        }
    }
}
