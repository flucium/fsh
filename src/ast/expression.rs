use super::*;

#[derive(Debug, Clone,PartialEq, Serialize)]
pub enum Expression {
    Null,
    String(String),
    Boolean(bool),
    Identifier(String),
    Number(isize),
    FileDescriptor(i32),
}

impl FshAst for Expression {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
