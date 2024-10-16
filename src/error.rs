use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Other,

    InvalidSyntax,
    PermissionDenied,
    NotFound,
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
        }
    }
}

impl AsRef<ErrorKind> for ErrorKind {
    fn as_ref(&self) -> &ErrorKind {
        self
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub const NOT_IMPLEMENTED: Error = Error {
        kind: ErrorKind::NotImplemented,
        message: String::new(),
    };

    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Error {
        let message = message.into();
        Self { kind, message }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}