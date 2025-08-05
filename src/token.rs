/// Represents a token in FSH.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// End of File.
    EOF,

    /// A semicolon (`;`), used to separate discrete syntactic elements.
    Semicolon,

    /// An ampersand (`&`), often used to modify execution behavior (e.g., run processes in the background).
    Ampersand,

    /// A pipe (`|`), used to express sequential data flow between elements.
    Pipe,

    /// A single equals sign (`=`), used for assignment operations.
    ///
    /// Note: Comparison operations (e.g., `==`) are handled separately.
    Equal,

    /// A less-than sign (`<`), which may be used either as an input redirection
    /// symbol or as a comparison operator depending on syntactic context.
    LessThan,

    /// A greater-than sign (`>`), which may be used either as an output redirection
    /// symbol or as a comparison operator depending on syntactic context.
    GreaterThan,

    /// Represents an explicit null token.
    Null,

    /// A string literal, e.g., `text` or `"Hello, World!"`, `'0123'`.
    String(String),

    /// A shell variable identifier, e.g., `$USER` or `$PATH`.
    Identifier(String),

    /// A boolean literal (`true` or `false`).
    Boolean(bool),

    /// An integer literal.
    Number(isize),

    /// A file descriptor literal, e.g., `@0` or `@3`.
    FileDescriptor(i32),
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}
