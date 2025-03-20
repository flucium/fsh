use super::FshAst;
use serde::Serialize;

/// Represents an expression in the Fsh AST.
///
/// The `Expression` enum defines various types of expressions that can appear
/// in the abstract syntax tree (AST).
#[derive(Debug,Clone, PartialEq, Serialize)]
pub enum Expression {
    /// Represents a null value.
    Null,

    /// Represents a string value.
    String(String),

    /// Represents a numeric value.
    Number(isize),

    /// Represents a boolean value (`true` or `false`).
    Boolean(bool),

    /// Represents an identifier, such as a variable or function name.
    Identifier(String),

    /// Represents a file descriptor.
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
