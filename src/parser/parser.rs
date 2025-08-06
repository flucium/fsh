use crate::{
    ast::statement::*, error::Error, lexer::Lexer, preprocessor::preprocess, result::Result,
    token::Token,
};

use super::lite_parser;

pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    index: usize,
    length: usize,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            lexer: Lexer::new(preprocess(source)),
            tokens: Vec::new(),
            index: 0,
            length: 0,
        }
    }

    fn collect(&mut self) -> Result<()> {
        self.tokens.clear();

        loop {
            let token = self.lexer.next()?;

            match token {
                Token::Semicolon | Token::EOF => break,

                Token::Ampersand => {
                    self.tokens.push(token);
                }

                _ => self.tokens.push(token),
            }
        }

        self.index = 0;
        self.length = self.tokens.len();

        Ok(())
    }

    fn parse_assignment(&mut self) -> Result<Assignment> {
        let tokens = self
            .tokens
            .get(self.index..self.index + 3)
            .map(|tokens| TryInto::<&[Token; 3]>::try_into(tokens).unwrap())
            .ok_or(Error::NOT_IMPLEMENTED)?;

        let assignment = lite_parser::parse_assignment(tokens)?;

        self.index += 3;

        Ok(assignment)
    }

    fn parse_command(&mut self) -> Result<Command> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::NOT_IMPLEMENTED)?;

        let command = lite_parser::parse_command(tokens)?;

        self.index = self.length;

        Ok(command)
    }

    fn parse_pipe(&mut self) -> Result<Pipe> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::NOT_IMPLEMENTED)?;

        let pipe = lite_parser::parse_pipe(tokens)?;

        self.index = self.length;

        Ok(pipe)
    }

    pub fn parse(&mut self) -> Result<Statement> {
        let mut sequence = Sequence::new();

        loop {
            if self.index == 0 || self.index == self.length {
                self.collect()?;
            }

            if self.length == 0 {
                break;
            }

            // let statement = self
            //     .parse_assignment()
            //     .map(Statement::Assignment)
            //     .map(|statement| {
            //         let mut sequence2 = Sequence::new();
            //         sequence2.push_back(statement);
            //         Statement::Sequence(sequence2)
            //     })
            //     .or_else(|_| {
            //         self.parse_command()
            //             .map(Statement::Command)
            //             .map(|statement| {
            //                 let mut sequence3 = Sequence::new();
            //                 sequence3.push_back(statement);
            //                 Statement::Sequence(sequence3)
            //             })
            //     })
            //     .or_else(|_| self.parse_pipe().map(Statement::Pipe))?;

            let statement = self
                .parse_assignment()
                .map(Statement::Assignment)
                .or_else(|_| self.parse_command().map(Statement::Command))
                .or_else(|_| self.parse_pipe().map(Statement::Pipe))?;

            sequence.push_back(statement);
        }

        Ok(Statement::Sequence(sequence))
    }
}
