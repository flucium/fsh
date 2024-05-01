use super::{FshAst, expr::*};
use serde::Serialize;

/// Represents a assignment statement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Assign {
    pub key: Expr,
    pub value: Expr,
}

impl FshAst for Assign {
    /// Converts the assignment to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

/// Represents a redirect statement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Redirect {
    pub left: Expr,
    pub right: Expr,
    pub operator: RedirectOperator,
}

impl FshAst for Redirect {
    
    /// Converts the redirect to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

/// Represents a redirect operator.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedirectOperator {
    Gt,
    Lt,
}

impl FshAst for RedirectOperator {

    /// Converts the redirect operator to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

/// Represents a command statement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Command {
    pub expr: Expr,
    pub args: Vec<Expr>,
    pub redirects: Vec<Redirect>,
    pub background: bool,
}

impl FshAst for Command {

    /// Converts the command to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}


/// Represents a statement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Statement {
    Command(Command),
    Assign(Assign),
}

impl FshAst for Statement {
    
    /// Converts the statement to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        match self {
            Statement::Command(command) => command.to_json(is_pretty),
            Statement::Assign(assign) => assign.to_json(is_pretty),
        }
    }
}