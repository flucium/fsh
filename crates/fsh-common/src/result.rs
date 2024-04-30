use crate::error;

/// A type alias for `Result<T, error::Error>`.
pub type Result<T> = core::result::Result<T, error::Error>;