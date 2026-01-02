#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Unknow,
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorKind::NotImplemented => "not implemented",
            ErrorKind::Internal => "internal",
            ErrorKind::Unknow => "unknow",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::NotImplemented => "not implemented".to_string(),
            ErrorKind::Internal => "internal".to_string(),
            ErrorKind::Unknow => "unknow".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        let message = message.into();

        Self { kind, message }
    }

    pub const NOT_IMPLEMENTED: Error = Error {
        kind: ErrorKind::NotImplemented,
        message: String::new(),
    };

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
