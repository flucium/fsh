pub mod expression;
pub mod pipe;
pub mod statement;

use serde::Serialize;
use std::collections::VecDeque;

pub trait FshAst {
    fn to_json(&self) -> String;

    fn to_json_pretty(&self) -> String;
}

#[derive(Debug, Serialize)]
pub enum Node {
    Block(Block),
    Statement(statement::Statement),
    Pipe(pipe::Pipe),
}

impl FshAst for Node {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct Block(VecDeque<Node>);

impl Block {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, node: Node) {
        self.0.push_back(node);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.0.pop_front()
    }
}

impl FshAst for Block {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
