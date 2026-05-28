use core::time::Duration;

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

    /// Queries the hardware directly to see if the lamp is currently burning.
    /// This bypasses any local memory and checks reality.
    pub fn is_powered_on(&mut self) -> Result<bool, ProjectorError> {
        // Send the explicit query: "* 0 Lamp ?"
        self.send_command(Command::Ask(Query::Lamp1State))?;

        // Wait for the response buffer
        let response = self.read_response_string()?;

        // Return a solid boolean based strictly on the hardware's reply
        match response.trim() {
            "*Lamp 1" => Ok(true),
            "*Lamp 0" => Ok(false),
            _ => Err(ProjectorError::ParseError),
        }
    }

    pub fn send_raw_command(&mut self, cmd_string: &str) -> Result<(), ProjectorError> {
        let bytes = cmd_string.as_bytes();
        self.port.write_all(bytes)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn power_on(&mut self) -> Result<(), ProjectorError> {
        self.send_command(Command::Power(PowerState::On))
    }

    pub fn power_off(&mut self) -> Result<(), ProjectorError> {
        self.send_command(Command::Power(PowerState::Off))
    }

    fn read_response_string(&mut self) -> Result<String, ProjectorError> {
        // Allocate a buffer to store the incoming bytes.
        let mut buffer: Vec<u8> = vec![0; 128];

        // Read from the serial port. This will block up to the timeout configured in `connect()`.
        match self.port.read(buffer.as_mut_slice()) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Err(ProjectorError::Timeout);
                }

                // Convert the raw bytes from the wire into a valid UTF-8 string
                String::from_utf8(buffer[..bytes_read].to_vec())
                    .map_err(|_| ProjectorError::ParseError)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Explicitly catch OS-level serial timeouts
                Err(ProjectorError::Timeout)
            }
            Err(e) => {
                // Wrap any other standard I/O hardware errors
                Err(ProjectorError::StdIo(e))
            }
        }
    }
}
