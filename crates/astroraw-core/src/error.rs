use thiserror::Error;

pub type Result<T> = std::result::Result<T, AstroError>;

#[derive(Debug, Error)]
pub enum AstroError {
    #[error("The RAW file has gone somewhere else: {0}")]
    FileNotFound(String),

    #[error("This file appears to be mostly harmless, but not a supported RAW format: {0}")]
    UnsupportedFormat(String),

    #[error("Could not read RAW metadata: {0}")]
    RawReadError(String),

    #[error("FITS write failed: {0}")]
    FitsWriteError(String),

    #[error("Metadata JSON is invalid. The Guide would not approve: {0}")]
    InvalidSession(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}
