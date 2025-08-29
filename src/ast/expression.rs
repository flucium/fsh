use super::FshAst;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expression {
    
    Null,

    String(String),

    Number(isize),

    Boolean(bool),

    Identifier(String),

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

