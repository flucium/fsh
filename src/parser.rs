use crate::{
    ast::{expression::*, pipe::*, statement::*, Block, Node},
    error::*,
    lexer::Lexer,
    preprocessor::preprocess,
    result::Result,
    token::Token,
    utils::recursion_split,
};

/// Parses a source string into an abstract syntax tree (AST).
///
/// The `Parser` struct provides functionality to tokenize and parse a source string
/// into a structured representation. It supports parsing assignments, commands,
/// pipes, and blocks.
// -----
// # Fields
// - `lexer`: A `Lexer` instance for tokenizing the input.
// - `tokens`: A vector of tokens collected during parsing.
// - `index`: The current position in the token list.
// - `length`: The total number of tokens in the current batch.
pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    index: usize,
    length: usize,
}

impl Parser {
    /// Creates a new `Parser` from the given source string.
    ///
    /// # Arguments
    /// - `source`: The input source string to parse.
    ///
    /// # Returns
    /// - A `Parser` instance initialized with the provided source.s
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            lexer: Lexer::new(preprocess(source)),
            tokens: Vec::new(),
            index: 0,
            length: 0,
        }
    }

    /// Collects tokens from the lexer until a semicolon (`;`), ampersand (`&`), or EOF is reached.
    ///
    /// This method clears the current token list and refills it with the next batch
    /// of tokens from the lexer.
    ///
    /// # Errors
    /// - Returns an error if tokenization fails.
    fn collect(&mut self) -> Result<()> {
        self.tokens.clear();

        loop {
            let token = self.lexer.next()?;

            match token {
                Token::Semicolon | Token::EOF => break,

                Token::Ampersand => {
                    self.tokens.push(token);
                    break;
                }

                _ => self.tokens.push(token),
            }
        }

        self.index = 0;
        self.length = self.tokens.len();

        Ok(())
    }

    /// Parses an assignment statement from the collected tokens.
    ///
    /// The assignment must follow the format `IDENTIFIER = VALUE`.
    ///
    /// # Returns
    /// - An `Assignment` struct representing the parsed assignment.
    ///
    /// # Errors
    /// - Returns an error if the token sequence does not match the expected format.
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

    /// Parses a command statement from the collected tokens.
    ///
    /// A command consists of a name, optional arguments, and optional redirections.
    ///
    /// # Returns
    /// - A `Command` struct representing the parsed command.
    ///
    /// # Errors
    /// - Returns an error if the token sequence does not match a valid command format.
    fn parse_command(&mut self) -> Result<Command> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::NOT_IMPLEMENTED)?;

        let command = lite_parser::parse_command(tokens)?;

        self.index = self.length;

        Ok(command)
    }

    /// Parses a pipe from the collected tokens.
    ///
    /// A pipe connects multiple commands using the pipe operator (`|`).
    ///
    /// # Returns
    /// - A `Pipe` struct representing the parsed pipe.
    ///
    /// # Errors
    /// - Returns an error if the token sequence does not match a valid pipe format.s
    fn parse_pipe(&mut self) -> Result<Pipe> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::new(ErrorKind::InvalidSyntax, "invalid pipe"))?;

        let pipe = lite_parser::parse_pipe(tokens)?;

        self.index = self.length;

        Ok(pipe)
    }

    /// Parses the input into an abstract syntax tree (AST).
    ///
    /// This method repeatedly collects tokens and parses them into nodes, which
    /// are added to a `Block`. The parsing process continues until all tokens are
    /// consumed.
    ///
    /// # Returns
    /// - A `Node` representing the root of the parsed AST.
    ///
    /// # Errors
    /// - Returns an error if any parsing step fails.
    pub fn parse(&mut self) -> Result<Node> {
        let mut block = Block::new();

        loop {
            if self.index == 0 || self.index == self.length {
                self.collect()?;
            }

            if self.length == 0 {
                break;
            }

            let node = self
                .parse_assignment()
                .map(Statement::Assignment)
                .map(Node::Statement)
                .or_else(|_| {
                    self.parse_command()
                        .map(Statement::Command)
                        .map(Node::Statement)
                })
                .or_else(|_| self.parse_pipe().map(Node::Pipe))?;

            block.push(node);
        }

        Ok(Node::Block(block))
    }
}

/// Provides lightweight parsing utilities for individual constructs.
///
/// The `lite_parser` module contains helper functions for parsing specific
/// tokens or constructs, such as expressions, assignments, and commands.
pub mod lite_parser {

    use super::*;

    /// Parses a null token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::Null` if the token represents a null value.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid null token.
    pub fn parse_null(token: &Token) -> Result<Expression> {
        match token {
            Token::Null => Ok(Expression::Null),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'null' keyword, but encountered an invalid token",
            )),
        }
    }

    /// Parses a string token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::String` if the token represents a string value.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid string token.
    pub fn parse_string(token: &Token) -> Result<Expression> {
        match token {
            Token::String(s) => Ok(Expression::String(s.clone())),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'string' token, but encountered an invalid token",
            )),
        }
    }

    /// Parses an identifier token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::Identifier` if the token represents an identifier.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid identifier token.
    pub fn parse_identifier(token: &Token) -> Result<Expression> {
        match token {
            Token::Identifier(s) => Ok(Expression::Identifier(s.clone())),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'identifier' token, but encountered an invalid token",
            )),
        }
    }

    /// Parses a boolean token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::Boolean` if the token represents a boolean value.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid boolean token.
    pub fn parse_boolean(token: &Token) -> Result<Expression> {
        match token {
            Token::Boolean(b) => Ok(Expression::Boolean(*b)),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'boolean' token, but encountered an invalid token",
            )),
        }
    }

    /// Parses a number token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::Number` if the token represents a number.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid number token.
    pub fn parse_number(token: &Token) -> Result<Expression> {
        match token {
            Token::Number(n) => Ok(Expression::Number(*n)),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'number' token, but encountered an invalid token",
            )),
        }
    }

    /// Parses a file descriptor token into an expression.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression::FileDescriptor` if the token represents a file descriptor.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid file descriptor token.
    pub fn parse_file_descriptor(token: &Token) -> Result<Expression> {
        match token {
            Token::FileDescriptor(n) => Ok(Expression::FileDescriptor(*n)),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 'file descriptor' token, but encountered an invalid token",
            )),
        }
    }

    /// Parses an assignment from a sequence of tokens.
    ///
    /// # Arguments
    /// - `tokens`: A slice of tokens representing the assignment.
    ///
    /// # Returns
    /// - An `Assignment` struct representing the parsed assignment.
    ///
    /// # Errors
    /// - Returns an error if the token sequence is not a valid assignment.
    pub fn parse_assignment(tokens: &[Token; 3]) -> Result<Assignment> {
        if tokens[1] != Token::Equal {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected '=' in assignment, but encountered an invalid token",
            ))?;
        }

        let identifier = parse_identifier(&tokens[0])?;

        let value = parse_null(&tokens[2])
            .or(parse_string(&tokens[2]))
            .or(parse_boolean(&tokens[2]))
            .or(parse_number(&tokens[2]))
            .or(parse_file_descriptor(&tokens[2]))
            .or_else(|_| {
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "invalid assignment value",
                ))
            })?;

        Ok(Assignment::new(identifier, value))
    }

    /// Parses an abbreviated redirect from two tokens.
    ///
    /// An abbreviated redirect assumes the format:
    /// - `>` or `<` (operator)
    /// - Right-hand side (e.g., file name, identifier, or file descriptor)
    ///
    /// # Arguments
    /// - `tokens`: A slice of exactly two tokens representing the redirect.
    ///
    /// # Returns
    /// - A `Redirect` struct representing the parsed redirect.
    ///
    /// # Errors
    /// - Returns an error if the tokens do not match the expected format.
    fn parse_abbreviated_redirect(tokens: &[Token; 2]) -> Result<Redirect> {
        let (left, operator) = match tokens[0] {
            Token::GreaterThan => (Expression::FileDescriptor(1), RedirectOperator::GreaterThan),

            Token::LessThan => (Expression::FileDescriptor(0), RedirectOperator::LessThan),

            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected redirect operator, but encountered an invalid token",
            ))?,
        };

        let right = parse_string(&tokens[1])
            .or(parse_identifier(&tokens[1]))
            .or(parse_number(&tokens[1]))
            .or(parse_file_descriptor(&tokens[1]))
            .or_else(|_| {
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "invalid redirect right-hand side",
                ))
            })?;

        Ok(Redirect::new(operator, left, right))
    }

    /// Parses a normal redirect from three tokens.
    ///
    /// A normal redirect assumes the format:
    /// - Left-hand side (e.g., file descriptor)
    /// - `>` or `<` (operator)
    /// - Right-hand side (e.g., file name, identifier, or file descriptor)
    ///
    /// # Arguments
    /// - `tokens`: A slice of exactly three tokens representing the redirect.
    ///
    /// # Returns
    /// - A `Redirect` struct representing the parsed redirect.
    ///
    /// # Errors
    /// - Returns an error if the tokens do not match the expected format.
    fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<Redirect> {
        let operator = match tokens[1] {
            Token::GreaterThan => RedirectOperator::GreaterThan,

            Token::LessThan => RedirectOperator::LessThan,

            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected redirect operator, but encountered an invalid token",
            ))?,
        };

        let left = parse_file_descriptor(&tokens[0]).or_else(|_| {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid redirect left-hand side",
            ))
        })?;

        let right = parse_string(&tokens[2])
            .or(parse_identifier(&tokens[2]))
            .or(parse_number(&tokens[2]))
            .or(parse_file_descriptor(&tokens[2]))
            .or_else(|_| {
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "invalid redirect right-hand side",
                ))
            })?;

        Ok(Redirect::new(operator, left, right))
    }

    /// Parses a redirect from a sequence of tokens.
    ///
    /// The redirect can be either abbreviated (2 tokens) or normal (3 tokens).
    ///
    /// # Arguments
    /// - `tokens`: A slice of tokens representing the redirect.
    ///
    /// # Returns
    /// - A `Redirect` struct representing the parsed redirect.
    ///
    /// # Errors
    /// - Returns an error if the tokens do not match either the abbreviated or normal format.
    pub fn parse_redirect(tokens: &[Token]) -> Result<Redirect> {
        match tokens.len() {
            2 => parse_abbreviated_redirect(tokens.try_into().unwrap()),
            3 => parse_normal_redirect(tokens.try_into().unwrap()),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "invalid redirect")),
        }
    }

    /// Parses a command name from a token.
    ///
    /// The command name must be a valid string, identifier, or number.
    ///
    /// # Arguments
    /// - `token`: The token to parse.
    ///
    /// # Returns
    /// - An `Expression` representing the command name.
    ///
    /// # Errors
    /// - Returns an error if the token is not a valid command name.
    fn parse_command_name(token: &Token) -> Result<Expression> {
        parse_string(token)
            .or(parse_identifier(token).or(parse_number(token)))
            .or_else(|_| {
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "expected command name token, but encountered an invalid token",
                ))
            })
    }

    /// Parses the arguments and redirects for a command.
    ///
    /// This function extracts command arguments, I/O redirects, and background execution
    /// markers from the tokens following a command name.
    ///
    /// # Arguments
    /// - `tokens`: A slice of tokens representing the command arguments and redirects.
    ///
    /// # Returns
    /// - A tuple containing:
    ///   - `Vec<Expression>`: The list of command arguments.
    ///   - `Vec<Redirect>`: The list of redirects.
    ///   - `Expression`: A boolean indicating whether the command should run in the background.
    ///
    /// # Errors
    /// - Returns an error if any token is invalid or improperly positioned.
    fn parse_command_arguments(
        tokens: &[Token],
    ) -> Result<(Vec<Expression>, Vec<Redirect>, Expression)> {
        let mut arguments = Vec::with_capacity(tokens.len());

        let mut redirects = Vec::with_capacity(tokens.len());

        let mut is_background = Expression::Boolean(false);

        let len = tokens.len();

        let mut skip_count = 0;

        for (i, token) in tokens.iter().enumerate() {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }

            match token {
                Token::GreaterThan | Token::LessThan => {
                    redirects.push(parse_redirect(&tokens[i..i + 2])?);
                    skip_count = 1;
                }

                Token::FileDescriptor(_) => {
                    redirects.push(parse_redirect(&tokens[i..i + 3])?);
                    skip_count = 2;
                }

                Token::Ampersand => {
                    if i == len - 1 {
                        is_background = Expression::Boolean(true);
                        break;
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidSyntax,
                            "unexpected ampersand position",
                        ))?;
                    }
                }
                _ => {
                    arguments.push(
                        parse_number(token)
                            .or(parse_identifier(token).or(parse_string(token)))
                            .or_else(|_| {
                                Err(Error::new(
                                    ErrorKind::InvalidSyntax,
                                    "invalid argument token",
                                ))
                            })?,
                    );
                }
            }
        }

        Ok((arguments, redirects, is_background))
    }

    /// Parses a command from a sequence of tokens.
    ///
    /// A command consists of:
    /// - A name (e.g., an identifier or string).
    /// - Zero or more arguments and redirects.
    /// - An optional background execution marker (`&`).
    ///
    /// # Arguments
    /// - `tokens`: A slice of tokens representing the command.
    ///
    /// # Returns
    /// - A `Command` struct representing the parsed command.
    ///
    /// # Errors
    /// - Returns an error if the tokens do not match the expected command format.
    pub fn parse_command(tokens: &[Token]) -> Result<Command> {
        let name = parse_command_name(&tokens[0])?;

        let (arguments, redirects, is_background) = parse_command_arguments(&tokens[1..])?;

        Ok(Command::new(name, arguments, redirects, is_background))
    }

    /// Parses a pipe from a sequence of tokens.
    ///
    /// A pipe connects multiple commands using the pipe operator (`|`).
    /// Each segment of the pipe is parsed as an individual command.
    ///
    /// # Arguments
    /// - `tokens`: A slice of tokens representing the pipe.
    ///
    /// # Returns
    /// - A `Pipe` struct representing the parsed pipe.
    ///
    /// # Errors
    /// - Returns an error if the tokens do not form a valid pipe structure.
    pub fn parse_pipe(tokens: &[Token]) -> Result<Pipe> {
        if tokens.len() < 3 {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid pipe syntax; insufficient tokens",
            ))?
        }

        let mut pipe = Pipe::new();

        for tokens in recursion_split(&Token::Pipe, tokens) {
            pipe.push(parse_command(&tokens)?);
        }

        // Ok(Pipe::from(
        //     recursion_split(&Token::Pipe, tokens)
        //         .iter()
        //         .map(|tokens| parse_command(tokens))
        //         .collect::<Result<Vec<_>>>()?,
        // ))

        Ok(pipe)
    }
}
