use super::FshAst;
use serde::Serialize;

/// Represents a literal or expression in the FSH AST.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expression {
    /// A null value.
    Null,

    /// A string literal.
    String(String),

    /// A numeric literal.
    Number(isize),

    /// A boolean literal (`true` or `false`).
    Boolean(bool),

    /// An identifier, such as a variable name.
    Identifier(String),

    /// A file descriptor (e.g., `@1` or `@2`).
    FileDescriptor(i32),
}

impl FshAst for Expression {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}
