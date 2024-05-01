mod expr;
mod pipe;
mod statement;

use serde::Serialize;
use std::collections::VecDeque;

//pub use
pub use expr::*;
pub use pipe::*;
pub use statement::*;

pub trait FshAst {
    fn to_json(&self, is_pretty: bool) -> String;
}

/// Represents an abstract syntax tree.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Ast {
    Block(Block),
    Statement(Statement),
    Pipe(Pipe),
}

impl Ast {
    /// Creates a new root AST.
    pub fn block(block: Block) -> Self {
        Ast::Block(block)
    }
}

impl FshAst for Ast {
    /// Converts the AST to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

/// Represents a block of ASTs.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Block (VecDeque<Ast>);

impl Block {
    /// Creates a new block.
    pub fn new() -> Self {
        Block(VecDeque::new())
    }

    /// Returns true if the block is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pushes an AST to the back of the block.
    pub fn push_back(&mut self, ast: Ast) {
        self.0.push_back(ast);
    }

    /// Pops an AST from the front of the block.
    pub fn pop_front(&mut self) -> Option<Ast> {
        self.0.pop_front()
    }
}

impl FshAst for Block {
    /// Converts the block to a JSON string.
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(&self.0).unwrap()
        } else {
            serde_json::to_string(&self.0).unwrap()
        }
    }
}
