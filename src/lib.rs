pub mod ast;
pub mod error;
pub mod execute;
pub mod manifest;
pub mod parser;
pub mod result;
pub mod sh_vars;
pub mod state;
pub mod terminal;

//
// pub crate
//
pub(crate) mod builtin;
pub(crate) mod lexer;
pub(crate) mod pipe;
pub(crate) mod preprocessor;
pub(crate) mod process_handler;
pub(crate) mod token;
pub(crate) mod utils;
