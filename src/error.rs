const FSH_ERROR_LABEL: &str = "fsh";
const FSH_ERROR_LABEL_COLON: char = ':';
const FSH_ERROR_LABEL_WHITESPACE: char = ' ';

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    NotImplemented,
    Internal,
    Other,
    Interrupted,
    PermissionDenied,
    InvalidInput,
    InvalidSyntax,
    InvalidPath,
    InvalidFileDescriptor,
    ExecutionFailed,
    NotFound,
    NotAFile,
    NotADirectory,
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotImplemented => "not implemented",
            Self::Internal => "internal",
            Self::Other => "other",
            Self::Interrupted => "interrupted",
            Self::PermissionDenied => "permission denied",
            Self::InvalidInput => "invalid input",
            Self::InvalidSyntax => "invalid syntax",
            Self::InvalidPath => "invalid path",
            Self::InvalidFileDescriptor => "invalid file descriptor",
            Self::ExecutionFailed => "execution failed",
            Self::NotFound => "not found",
            Self::NotAFile => "not a file",
            Self::NotADirectory => "not a directory",
        }
    }
}

impl AsRef<ErrorKind> for ErrorKind {
    fn as_ref(&self) -> &ErrorKind {
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub const NOT_IMPLEMENTED: Error = Error {
        kind: ErrorKind::NotImplemented,
        message: String::new(),
    };

    pub const INTERNAL: Error = Error {
        kind: ErrorKind::Internal,
        message: String::new(),
    };

    pub const OTHER: Error = Error {
        kind: ErrorKind::Other,
        message: String::new(),
    };

    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
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

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        err.into()
    }
}

pub fn errformat(err: &Error, arg: Option<&str>) -> String {
    if let Some(arg) = arg {
        format!(
            "{FSH_ERROR_LABEL}{FSH_ERROR_LABEL_COLON}{FSH_ERROR_LABEL_WHITESPACE}{arg}{FSH_ERROR_LABEL_COLON}{FSH_ERROR_LABEL_WHITESPACE}{}",
            err.message()
        )
    } else {
        format!(
            "{FSH_ERROR_LABEL}{FSH_ERROR_LABEL_COLON}{FSH_ERROR_LABEL_WHITESPACE}{}",
            err.message()
        )
    }
}
