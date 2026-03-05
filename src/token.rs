/// Lexical token produced by the parser's tokenizer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// End-of-input marker.
    EOF,

    /// `;`
    Semicolon,

    /// `&`
    Ampersand,

    /// `|`
    Pipe,

    /// `=`
    Assign,

    /// `==`
    Equal,

    /// `<`
    LessThan,

    /// `>`
    GreaterThan,

    /// `null`
    Null,

    /// String literal value.
    String(String),

    /// Identifier name.
    Identifier(String),

    /// Boolean literal value.
    Boolean(bool),

    /// Numeric literal value.
    Number(isize),

    /// File descriptor number.
    FileDescriptor(i32),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::EOF => "eof".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Ampersand => "&".to_string(),
            Token::Pipe => "|".to_string(),
            Token::Equal => "==.".to_string(),
            Token::Assign => "=".to_string(),
            Token::LessThan => "<".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::Null => "null".to_string(),
            Token::String(string) => string.to_string(),
            Token::Identifier(identifier) => identifier.to_string(),
            Token::Boolean(boolean) => boolean.to_string(),
            Token::Number(number) => number.to_string(),
            Token::FileDescriptor(fd) => fd.to_string(),
        }
    }
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}
