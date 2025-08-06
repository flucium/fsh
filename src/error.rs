/// Represents the kind or type of an error.
///
/// Used internally to categorize the nature of the failure.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Other,
}

impl ErrorKind {
    /// Returns a string representation of the error kind.
    ///
    /// # Returns
    /// A static string describing the error kind (e.g., `"not implemented"`).
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotImplemented => "not implemented",
            Self::Internal => "internal",
            Self::Other => "other",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            Self::NotImplemented => String::from("not implemented"),
            Self::Internal => String::from("internal"),
            Self::Other => String::from("other"),
        }
    }
}

impl AsRef<ErrorKind> for ErrorKind {
    fn as_ref(&self) -> &ErrorKind {
        self
    }
}

/// Represents a structured error with a kind and optional message.
///
/// This type is used to describe recoverable or fatal failures
/// in a consistent way throughout the shell.
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    /// A predefined constant representing a generic "not implemented" error.
    ///
    /// The message is left empty.
    pub const NOT_IMPLEMENTED: Error = Error {
        kind: ErrorKind::NotImplemented,
        message: String::new(),
    };

    /// Creates a new error from the given kind and message.
    ///
    /// # Arguments
    /// - `kind`: The kind of error.
    /// - `message`: A human-readable message describing the error.
    ///
    /// # Returns
    /// A new `Error` instance.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        let message = message.into();

        Self { kind, message }
    }

    /// Returns a reference to the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the message associated with this error.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}
