use super::lexer::Lexer;
use super::lite_parser::{parse_assign, parse_command, parse_pipe};
use super::token::Token;
use super::utils::recursion_split;
use fsh_ast::*;
use fsh_common::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Parser(Lexer, Block);

impl Parser {
    /// Creates a new parser from the given input.
    pub fn new(input: &str) -> Self {
        Self(Lexer::new(input), Block::new())
    }

    /// Parses the input and returns the AST.
    pub fn parse(&mut self) -> Result<Ast> {
        let mut tokens = self.0.tokenize()?;

        if tokens.pop() != Some(Token::EOF) {
            Err(Error::new(ErrorKind::SyntaxError, "unexpected EOF"))?;
        }

        let entries = recursion_split(&Token::Semicolon, &tokens);

        for tokens in entries {
            if tokens.is_empty() {
                continue;
            }

            if tokens.contains(&Token::Pipe) {
                self.1.push_back(Ast::Pipe(parse_pipe(&tokens)?));

                continue;
            }

            if tokens.contains(&Token::Assign) && tokens.len() == 3 {
                self.1
                    .push_back(Ast::Statement(Statement::Assign(parse_assign(
                        &tokens.try_into().unwrap(),
                    )?)));

                continue;
            }

            self.1
                .push_back(Ast::Statement(Statement::Command(parse_command(&tokens)?)));
        }

        Ok(Ast::block(self.1.to_owned()))
    }
}