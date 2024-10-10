use super::{statement::*, *};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Serialize)]
pub struct Pipe(VecDeque<Command>);

impl Pipe {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, command: Command) {
        self.0.push_back(command);
    }

    pub fn pop(&mut self) -> Option<Command> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

impl From<VecDeque<Command>> for Pipe {
    fn from(commands: VecDeque<Command>) -> Self {
        Self(commands)
    }
}

impl From<Vec<Command>> for Pipe {
    fn from(commands: Vec<Command>) -> Self {
        Self(commands.into())
    }
}

impl FshAst for Pipe {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
