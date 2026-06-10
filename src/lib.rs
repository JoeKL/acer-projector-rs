use core::{result::Result, time::Duration};
use std::io::Write;

use serialport::SerialPort;

pub mod commands;
pub mod enums;
pub mod error;

pub use commands::Command;
pub use error::ProjectorError;

use enums::{PowerState, Query};

pub struct Projector {
    port: Box<dyn SerialPort>,
}

impl Projector {
    pub fn connect(port_name: &str, bauds: u32, timeout_ms: u64) -> Result<Self, ProjectorError> {
        let port = serialport::new(port_name, bauds)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(Duration::from_millis(timeout_ms))
            .open()?;

        Ok(Projector { port })
    }

    pub fn send_command(&mut self, cmd: Command) -> Result<(), ProjectorError> {
        let cmd_string = cmd.to_string();
        let bytes = cmd_string.as_bytes();

        self.port.write_all(bytes)?;
        self.port.flush()?;

        Ok(())
    }

    pub fn send_raw_command(&mut self, cmd_string: &str) -> Result<(), ProjectorError> {
        let bytes = cmd_string.as_bytes();
        self.port.write_all(bytes)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn send_query(&mut self, query: Query) -> Result<String, ProjectorError> {
        self.send_command(Command::Ask(query))?;
        let response = self.read_response_string()?;
        Ok(response)
    }

    /// Queries the hardware directly to see if the lamp is currently burning.
    /// This bypasses any local memory and checks reality.
    pub fn is_powered_on(&mut self) -> Result<bool, ProjectorError> {
        let response = self.send_query(Query::Lamp1State)?;

        match response.as_str() {
            "Lamp 1" => Ok(true),
            "Lamp 0" => Ok(false),
            _ => Err(ProjectorError::ParseError),
        }
    }

    pub fn power_on(&mut self) -> Result<(), ProjectorError> {
        self.send_command(Command::Power(PowerState::On))
    }

    pub fn power_off(&mut self) -> Result<(), ProjectorError> {
        self.send_command(Command::Power(PowerState::Off))
    }

    fn read_response_string(&mut self) -> Result<String, ProjectorError> {
        let mut buffer: Vec<u8> = vec![0; 128];
        let mut total_read = 0;

        loop {
            if total_read >= buffer.len() {
                break;
            }

            match self.port.read(&mut buffer[total_read..]) {
                Ok(0) => {
                    if total_read == 0 {
                        return Err(ProjectorError::Timeout);
                    }
                    break;
                }
                Ok(bytes_read) => {
                    total_read += bytes_read;

                    if buffer[..total_read].contains(&b'\n') {
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if total_read == 0 {
                        return Err(ProjectorError::Timeout);
                    }
                    break;
                }
                Err(e) => {
                    return Err(ProjectorError::StdIo(e));
                }
            }
        }

        let raw_response = String::from_utf8((buffer[..total_read]).to_vec())
            .map_err(|_| ProjectorError::ParseError)?;

        // The response is expected to be in the format: "*000\rXXXXX\r" where XXXXX is the actual response we care about.
        let response = raw_response
            .split("\r")
            .nth(1)
            .ok_or(ProjectorError::ParseError)?
            .trim()
            .to_string();

        Ok(response)
    }
}
