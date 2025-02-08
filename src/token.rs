#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// End of File.
    EOF,

    /// Semicolon (;).
    Semicolon,

    /// Ampersand (&).
    Ampersand,

    /// Command pipe (|).
    Pipe,

    /// Assignment ($var = value).
    Equal,

    /// Less than (<).
    LessThan,

    /// Greater than (>).
    GreaterThan,

    /// Null token (null).
    Null,

    /// String token.
    String(String),

    /// Variable identifier token (&identifier).
    Identifier(String),

    /// Bool token (true, false).
    Boolean(bool),

    /// Number token.
    Number(isize),

    /// File descriptor token (@0).
    FileDescriptor(i32),
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}
