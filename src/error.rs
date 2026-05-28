use thiserror::Error;

/// Errors that can occur when interacting with the projector.
#[derive(Error, Debug)]
pub enum ProjectorError {
    /// Wraps errors originating from the serial port hardware or OS drivers.
    #[error("Serial port error: {0}")]
    IoError(#[from] serialport::Error),

    /// Wraps standard Rust I/O errors.
    #[error("I/O error: {0}")]
    StdIo(#[from] std::io::Error),

    /// Returned when the projector sends a response we don't understand.
    #[error("Failed to parse projector response")]
    ParseError,

    /// Returned when the projector takes too long to respond.
    #[error("Command timed out")]
    Timeout,
}
