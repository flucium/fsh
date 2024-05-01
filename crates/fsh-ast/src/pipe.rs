use super::{Command, FshAst};
use serde::Serialize;
use std::collections::VecDeque;

/// Represents a pipe.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Pipe (VecDeque<Command>);

impl Pipe {
    /// Creates a new pipe.
    pub fn new() -> Self {
        Pipe(VecDeque::new())
    }

    /// Returns true if the pipe is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pushes a command to the back of the pipe.
    pub fn push_back(&mut self, command: Command) {
        self.0.push_back(command);
    }

    /// Pops a command from the front of the pipe.
    pub fn pop_front(&mut self) -> Option<Command> {
        self.0.pop_front()
    }
}

impl FshAst for Pipe {
    /// Converts the pipe to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

impl From<VecDeque<Command>> for Pipe {

    /// Converts a vector of commands to a pipe.
    fn from(commands: VecDeque<Command>) -> Self {
        Pipe(commands)
    }
}

impl From<&[Command]> for Pipe {

    /// Converts a slice of commands to a pipe.
    fn from(commands: &[Command]) -> Self {
        Pipe(commands.iter().cloned().collect())
    }
}
