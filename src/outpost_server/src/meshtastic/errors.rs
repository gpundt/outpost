use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Meshtastic error: {0}")]
    Meshtastic(#[from] meshtastic::errors::Error),

    #[error("Serial port error: {0}")]
    SerialPort(#[from] tokio_serial::Error),

    #[error("Port no found: {0}")]
    PortNotFound(String),

    #[error("Already connected to a device")]
    AlreadyConnected,

    #[error("Not connected to any device")]
    NotConnected,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}
