#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Other,

    PermissionDenied,
    NotFound,
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
