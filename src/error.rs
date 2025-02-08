
/// Represents an error in the system.
///
/// The `Error` struct contains information about an error, including its kind
/// and a descriptive message.
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    /// The type of error that occurred.
    kind: ErrorKind,

    /// A human-readable message describing the error.
    message: String,
}

impl Error {
    pub const NOT_IMPLEMENTED: Error = Error {
        kind: ErrorKind::NotImplemented,
        message: String::new(),
    };

    /// Creates a new `Error` instance.
    ///
    /// # Arguments
    /// - `kind` - The type of error.
    /// - `message` - A descriptive message about the error.
    ///
    /// # Returns
    /// A new `Error` instance.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        let message = message.into();

        Self { kind, message }
    }

    /// Returns the kind of error.
    ///
    /// # Returns
    /// A reference to the `ErrorKind`.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the error message.
    ///
    /// # Returns
    /// A string slice containing the error message.s
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Other,
    InvalidSyntax,
    PermissionDenied,
    NotFound,

    // PipeUnavailable,
    // PipeBroken,
    // InvalidPipeState,
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotImplemented => "not implemented",
            Self::Internal => "internal error",
            Self::Other => "other error",
            Self::InvalidSyntax => "invalid syntax",
            Self::PermissionDenied => "permission denied",
            Self::NotFound => "not found",
            // Self::PipeUnavailable => "pipe unavailable",
            // Self::PipeBroken => "pipe broken",
            // Self::InvalidPipeState => "invalid pipe state",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            Self::NotImplemented => String::from("not implemented"),
            Self::Internal => String::from("internal error"),
            Self::Other => String::from("other error"),
            Self::InvalidSyntax => String::from("invalid syntax"),
            Self::PermissionDenied => String::from("permission denied"),
            Self::NotFound => String::from("not found"),
            // Self::PipeUnavailable => String::from("pipe unavailable"),
            // Self::PipeBroken => String::from("pipe broken"),
            // Self::InvalidPipeState => String::from("invalid pipe state"),
        }
    }
}

impl AsRef<ErrorKind> for ErrorKind {
    fn as_ref(&self) -> &ErrorKind {
        self
    }
}
