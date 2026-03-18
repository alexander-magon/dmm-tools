use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HID error: {0}")]
    Hid(#[from] hidapi::HidError),

    #[error("device not found (VID={vid:#06x}, PID={pid:#06x})")]
    DeviceNotFound { vid: u16, pid: u16 },

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("checksum mismatch: expected {expected:#06x}, got {actual:#06x}")]
    ChecksumMismatch { expected: u16, actual: u16 },

    #[error("timeout waiting for response")]
    Timeout,

    #[error("unknown mode: {0:#04x}")]
    UnknownMode(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
