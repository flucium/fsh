#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    Semicolon,
    Ampersand,
    Pipe,
    Equal,
    LessThan,
    // LessThanLessThan,
    GreaterThan,
    // GreaterThanGreaterThan,
    Null,
    String(String),
    Identifier(String),
    Boolean(bool),
    Number(isize),
    FileDescriptor(i32),
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}
