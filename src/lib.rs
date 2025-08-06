#![feature(test)]
extern crate test;

pub mod lexer;
pub mod preprocessor;
pub mod token;
pub mod result;
pub mod error;
pub mod sh_vars;
pub mod terminal;
pub mod builtin;
pub mod ast;
// pub mod process_handler;
pub mod state;
pub mod parser;
pub mod manifest;
pub mod execute;
pub mod profile;
pub mod utils;
pub mod prompt;