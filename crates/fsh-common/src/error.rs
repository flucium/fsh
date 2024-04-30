/// Error Kind
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Dummy,
    Internal,
    Other,
    Unknown,
    NotFound,
    NotAFile,
    NotADirectory,
    AlreadyExists,
    PermissionDenied,
    Interrupted,
    Failure,
    LexerError,
    SyntaxError,
    EngineError,
    BrokenPipe,
    InvalidInput,
}

impl ErrorKind {

    /// Returns a string representation of the error kind.
    pub const fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Dummy => "Dummy",
            ErrorKind::Internal => "Internal",
            ErrorKind::Other => "Other",
            ErrorKind::Unknown => "Unknown",
            ErrorKind::NotFound => "NotFound",
            ErrorKind::NotAFile => "NotAFile",
            ErrorKind::NotADirectory => "NotADirectory",
            ErrorKind::AlreadyExists => "AlreadyExists",
            ErrorKind::PermissionDenied => "PermissionDenied",
            ErrorKind::Interrupted => "Interrupted",
            ErrorKind::Failure => "Failure",
            ErrorKind::LexerError => "LexerError",
            ErrorKind::SyntaxError => "SyntaxError",
            ErrorKind::EngineError => "EngineError",
            ErrorKind::BrokenPipe => "BrokenPipe",
            ErrorKind::InvalidInput => "InvalidInput",
        }
    }

    /// Returns a lowercase string representation of the error kind.
    pub const fn as_str_lowercase(&self) -> &'static str {
        match self {
            ErrorKind::Dummy => "dummy",
            ErrorKind::Internal => "internal",
            ErrorKind::Other => "other",
            ErrorKind::Unknown => "unknown",
            ErrorKind::NotFound => "not found",
            ErrorKind::NotAFile => "not a file",
            ErrorKind::NotADirectory => "not a directory",
            ErrorKind::AlreadyExists => "already exists",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::Interrupted => "interrupted",
            ErrorKind::Failure => "failure",
            ErrorKind::LexerError => "lexer error",
            ErrorKind::SyntaxError => "syntax error",
            ErrorKind::EngineError => "engine error",
            ErrorKind::BrokenPipe => "broken pipe",
            ErrorKind::InvalidInput => "invalid input",
        }
    }
}

/// Error
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    // message: &'static str,
    message: String,
}

impl Error {

    /// Dummy error
    pub const DUMMY: Self = Self {
        kind: ErrorKind::Dummy,
        message: String::new(),
    };

    /// Internal error
    pub const INTERNAL: Self = Self {
        kind: ErrorKind::Internal,
        message: String::new(),
    };

    /// New error
    /// 
    /// Create a new error with the given kind and message.
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        let message = message.to_string();
        Self { kind, message }
    }

    /// Returns the error kind.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}
