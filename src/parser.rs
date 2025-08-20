use crate::{ast::statement::*, error::*, lexer::*, result::*, token::*};

/// A parser that converts tokens into statements.
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
    /// - `source` - Source code as a string.
    ///
    /// # Returns
    /// - A `Parser` initialized with the given source.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            lexer: Lexer::new(source.into()),
            tokens: Vec::new(),
            index: 0,
            length: 0,
        }
    }

    /// Collects tokens from the lexer until a semicolon or EOF is reached.
    ///
    /// # Returns
    /// - `Ok(())` if tokens were collected successfully.  
    /// - `Err(Error)` if tokenization fails.
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

    /// Parses an assignment statement from the current token slice.
    ///
    /// # Returns
    /// - `Ok(Assignment)` if the slice represents a valid assignment.  
    /// - `Err(Error)` if the syntax is invalid or not enough tokens remain.
    fn parse_assignment(&mut self) -> Result<Assignment> {
        let tokens = self
            .tokens
            .get(self.index..self.index + 3)
            .map(|tokens| TryInto::<&[Token; 3]>::try_into(tokens).unwrap())
            .ok_or(Error::new(
                ErrorKind::InvalidSyntax,
                "expected 3 tokens for assignment",
            ))?;

        let assignment = lite_parser::parse_assignment(tokens)?;

        self.index += 3;

        Ok(assignment)
    }

    /// Parses a command from the current token slice.
    ///
    /// # Returns
    /// - `Ok(Command)` if the slice represents a valid command.  
    /// - `Err(Error)` if the syntax is invalid.
    fn parse_command(&mut self) -> Result<Command> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid command token slice",
            ))?;

        let command = lite_parser::parse_command(tokens)?;

        self.index = self.length;

        Ok(command)
    }

    /// Parses a pipe expression from the current token slice.
    ///
    /// # Returns
    /// - `Ok(Pipe)` if the slice represents a valid pipe.  
    /// - `Err(Error)` if the syntax is invalid.
    fn parse_pipe(&mut self) -> Result<Pipe> {
        let tokens = self
            .tokens
            .get(self.index..self.length - self.index)
            .ok_or(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid pipe token slice",
            ))?;

        let pipe = lite_parser::parse_pipe(tokens)?;

        self.index = self.length;

        Ok(pipe)
    }

    /// Parses a full statement sequence from the input.
    ///
    /// A sequence may consist of:
    /// - Assignments
    /// - Commands
    /// - Pipes
    ///
    /// # Returns
    /// - `Ok(Statement)` representing the parsed sequence.  
    /// - `Err(Error)` if parsing fails.
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

pub mod lite_parser {
    use crate::{
        ast::{expression::*, statement::*},
        error::*,
        result::*,
        token::*,
    };

    /// Parses a null token into a `Null` expression.
    pub fn parse_null(token: &Token) -> Result<Expression> {
        match token {
            Token::Null => Ok(Expression::Null),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "expected null")),
        }
    }

    /// Parses a string token into a `String` expression.
    pub fn parse_string(token: &Token) -> Result<Expression> {
        match token {
            Token::String(s) => Ok(Expression::String(s.clone())),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "expected string")),
        }
    }

    /// Parses an identifier token into an `Identifier` expression.
    pub fn parse_identifier(token: &Token) -> Result<Expression> {
        match token {
            Token::Identifier(s) => Ok(Expression::Identifier(s.clone())),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "expected identifier")),
        }
    }

    /// Parses a boolean token into a `Boolean` expression.
    pub fn parse_boolean(token: &Token) -> Result<Expression> {
        match token {
            Token::Boolean(b) => Ok(Expression::Boolean(*b)),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "expected boolean")),
        }
    }

    /// Parses a number token into a `Number` expression.
    pub fn parse_number(token: &Token) -> Result<Expression> {
        match token {
            Token::Number(n) => Ok(Expression::Number(*n)),
            _ => Err(Error::new(ErrorKind::InvalidSyntax, "expected number")),
        }
    }

    /// Parses a file descriptor token into a `FileDescriptor` expression.
    pub fn parse_file_descriptor(token: &Token) -> Result<Expression> {
        match token {
            Token::FileDescriptor(n) => Ok(Expression::FileDescriptor(*n)),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected file descriptor",
            )),
        }
    }

    fn parse_assignment_value(token: &Token) -> Result<Expression> {
        parse_null(token)
            .or_else(|_| parse_string(token))
            .or_else(|_| parse_boolean(token))
            .or_else(|_| parse_number(token))
            .or_else(|_| parse_file_descriptor(token))
    }

    /// Parses an assignment statement from three tokens (`identifier = value`).
    ///
    /// # Arguments
    /// - `tokens` - An array of three tokens representing an assignment.
    ///
    /// # Returns
    /// - `Ok(Assignment)` if the tokens form a valid assignment.  
    /// - `Err(Error)` if the syntax is invalid.
    pub fn parse_assignment(tokens: &[Token; 3]) -> Result<Assignment> {
        if tokens[1] != Token::Equal {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "expected equal sign in assignment",
            ))?
        }

        let identifier = parse_identifier(&tokens[0])?;

        let value = parse_assignment_value(&tokens[2])?;

        Ok(Assignment::new(identifier, value))
    }

    fn parse_redirect_right(token: &Token) -> Result<Expression> {
        parse_string(token)
            .or_else(|_| parse_identifier(token))
            .or_else(|_| parse_number(token))
            .or_else(|_| parse_file_descriptor(token))
    }

    fn parse_abbreviated_redirect(tokens: &[Token; 2]) -> Result<Redirect> {
        let (left, operator) = match tokens[0] {
            Token::GreaterThan => (Expression::FileDescriptor(1), RedirectOperator::GreaterThan),
            Token::LessThan => (Expression::FileDescriptor(0), RedirectOperator::LessThan),
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid redirect operator",
            ))?,
        };

        let right = parse_redirect_right(&tokens[1])?;

        Ok(Redirect::new(operator, left, right))
    }

    fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<Redirect> {
        let operator = match tokens[1] {
            Token::GreaterThan => RedirectOperator::GreaterThan,
            Token::LessThan => RedirectOperator::LessThan,
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid redirect operator",
            ))?,
        };

        let left = parse_file_descriptor(&tokens[0])?;

        let right = parse_redirect_right(&tokens[2])?;

        Ok(Redirect::new(operator, left, right))
    }

    /// Parses a redirect from tokens.
    ///
    /// Supports abbreviated (2 tokens) and normal (3 tokens) redirect forms.
    ///
    /// # Arguments
    /// - `tokens` - The slice of tokens to parse.
    ///
    /// # Returns
    /// - `Ok(Redirect)` if the tokens form a valid redirect.  
    /// - `Err(Error)` if the syntax is invalid or the token count is unexpected.
    pub fn parse_redirect(tokens: &[Token]) -> Result<Redirect> {
        match tokens.len() {
            2 => {
                let arr: &[Token; 2] = tokens.try_into().map_err(|_| {
                    Error::new(
                        ErrorKind::InvalidSyntax,
                        "expected 2 tokens for abbreviated redirect",
                    )
                })?;
                parse_abbreviated_redirect(arr)
            }
            3 => {
                let arr: &[Token; 3] = tokens.try_into().map_err(|_| {
                    Error::new(
                        ErrorKind::InvalidSyntax,
                        "expected 3 tokens for normal redirect",
                    )
                })?;
                parse_normal_redirect(arr)
            }
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                "unexpected number of redirect tokens",
            ))?,
        }
    }

    fn parse_command_name(token: &Token) -> Result<Expression> {
        parse_string(token)
            .or(parse_identifier(token).or(parse_number(token)))
            .or_else(|_| Err(Error::new(ErrorKind::InvalidSyntax, "invalid command name")))
    }

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
                            "unexpected ampersand in argument list",
                        ))?
                    }
                }
                _ => {
                    arguments.push(
                        parse_number(token)
                            .or(parse_identifier(token).or(parse_string(token)))
                            .or_else(|_| {
                                Err(Error::new(
                                    ErrorKind::InvalidSyntax,
                                    "invalid command argument",
                                ))
                            })?,
                    );
                }
            }
        }

        Ok((arguments, redirects, is_background))
    }

    /// Parses a command from tokens.
    ///
    /// A command consists of:
    /// - A name (string, identifier, or number)
    /// - Zero or more arguments
    /// - Optional redirects
    /// - An optional background marker (`&`)
    ///
    /// # Arguments
    /// - `tokens` - A slice of tokens representing the command.
    ///
    /// # Returns
    /// - `Ok(Command)` if the tokens form a valid command.  
    /// - `Err(Error)` if the syntax is invalid.
    pub fn parse_command(tokens: &[Token]) -> Result<Command> {
        let name = parse_command_name(&tokens[0])?;

        let (arguments, redirects, is_background) = parse_command_arguments(&tokens[1..])?;

        Ok(Command::new(name, arguments, redirects, is_background))
    }

    /// Parses a pipe from tokens.
    ///
    /// A pipe must contain at least one command separated by `|`.
    ///
    /// # Arguments
    /// - `tokens` - A slice of tokens representing the pipe expression.
    ///
    /// # Returns
    /// - `Ok(Pipe)` if the tokens form a valid pipe.  
    /// - `Err(Error)` if the syntax is invalid or incomplete.
    pub fn parse_pipe(tokens: &[Token]) -> Result<Pipe> {
        if tokens.len() < 3 {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "pipe must contain at least one command",
            ))?
        }

        let mut pipe = Pipe::new();

        for tokens in recursion_split(&Token::Pipe, tokens) {
            pipe.push_back(parse_command(&tokens)?);
        }

        Ok(pipe)
    }

    // Splits tokens once at the given separator.
    fn split(place: &Token, tokens: &[Token]) -> (Vec<Token>, Vec<Token>) {
        if let Some(pos) = tokens.iter().position(|t| t == place) {
            (tokens[..pos].to_vec(), tokens[pos + 1..].to_vec())
        } else {
            (tokens.to_vec(), Vec::new())
        }
    }

    // Recursively splits tokens by the given separator.
    fn recursion_split(place: &Token, tokens: &[Token]) -> Vec<Vec<Token>> {
        let (left, right) = split(place, tokens);

        if right.is_empty() {
            vec![left]
        } else {
            let mut result = vec![left];
            result.extend(recursion_split(place, &right));
            result
        }
    }
}
