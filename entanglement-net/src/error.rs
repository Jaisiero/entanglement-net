use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("batch full: no room for {needed} bytes ({available} available)")]
    BatchFull { needed: usize, available: usize },

    #[error("malformed batch: truncated at offset {offset}")]
    MalformedBatch { offset: usize },

    #[error("unknown message type: 0x{0:04X}")]
    UnknownMessageType(u16),

    #[error("payload too small: expected {expected} bytes, got {actual}")]
    PayloadTooSmall { expected: usize, actual: usize },

    #[error("handler error: {0}")]
    HandlerError(String),
}
