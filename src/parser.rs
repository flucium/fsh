use crate::{
    ast::{expression::*, pipe::*, statement::*, Block, Node},
    error::*,
    lexer::Lexer,
    preprocessor::preprocess,
    result::Result,
    token::Token,
    utils::recursion_split,
};

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
                    break;
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

pub mod lite_parser {

    use super::*;

    pub fn parse_null(token: &Token) -> Result<Expression> {
        match token {
            Token::Null => Ok(Expression::Null),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_string(token: &Token) -> Result<Expression> {
        match token {
            Token::String(s) => Ok(Expression::String(s.clone())),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_identifier(token: &Token) -> Result<Expression> {
        match token {
            Token::Identifier(s) => Ok(Expression::Identifier(s.clone())),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_boolean(token: &Token) -> Result<Expression> {
        match token {
            Token::Boolean(b) => Ok(Expression::Boolean(*b)),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_number(token: &Token) -> Result<Expression> {
        match token {
            Token::Number(n) => Ok(Expression::Number(*n)),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_file_descriptor(token: &Token) -> Result<Expression> {
        match token {
            Token::FileDescriptor(n) => Ok(Expression::FileDescriptor(*n)),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    pub fn parse_assignment(tokens: &[Token; 3]) -> Result<Assignment> {
        if tokens[1] != Token::Equal {
            Err(Error::NOT_IMPLEMENTED)?
        }

        let identifier = parse_identifier(&tokens[0])?;

        let value = parse_null(&tokens[2])
            .or(parse_string(&tokens[2]))
            .or(parse_boolean(&tokens[2]))
            .or(parse_number(&tokens[2]))
            .or(parse_file_descriptor(&tokens[2]))?;

        Ok(Assignment::new(identifier, value))
    }

    fn parse_abbreviated_redirect(tokens: &[Token; 2]) -> Result<Redirect> {
        let (left, operator) = match tokens[0] {
            Token::GreaterThan => (Expression::FileDescriptor(1), RedirectOperator::GreaterThan),

            Token::LessThan => (Expression::FileDescriptor(0), RedirectOperator::LessThan),

            _ => Err(Error::NOT_IMPLEMENTED)?,
        };

        let right = parse_string(&tokens[1])
            .or(parse_identifier(&tokens[1]))
            .or(parse_number(&tokens[1]))
            .or(parse_file_descriptor(&tokens[1]))?;

        Ok(Redirect::new(operator, left, right))
    }

    fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<Redirect> {
        let operator = match tokens[1] {
            Token::GreaterThan => RedirectOperator::GreaterThan,

            Token::LessThan => RedirectOperator::LessThan,

            _ => Err(Error::NOT_IMPLEMENTED)?,
        };

        let left = parse_file_descriptor(&tokens[0])?;

        let right = parse_string(&tokens[2])
            .or(parse_identifier(&tokens[2]))
            .or(parse_number(&tokens[2]))
            .or(parse_file_descriptor(&tokens[2]))?;

        Ok(Redirect::new(operator, left, right))
    }

    pub fn parse_redirect(tokens: &[Token]) -> Result<Redirect> {
        match tokens.len() {
            2 => parse_abbreviated_redirect(tokens.try_into().unwrap()),
            3 => parse_normal_redirect(tokens.try_into().unwrap()),
            _ => Err(Error::NOT_IMPLEMENTED),
        }
    }

    fn parse_command_name(token: &Token) -> Result<Expression> {
        parse_string(token).or(parse_identifier(token).or(parse_number(token)))
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
                        Err(Error::NOT_IMPLEMENTED)?
                    }
                }
                _ => {
                    arguments.push(
                        parse_number(token).or(parse_identifier(token).or(parse_string(token)))?,
                    );
                }
            }
        }

        Ok((arguments, redirects, is_background))
    }

    pub fn parse_command(tokens: &[Token]) -> Result<Command> {
        let name = parse_command_name(&tokens[0])?;

        let (arguments, redirects, is_background) = parse_command_arguments(&tokens[1..])?;

        Ok(Command::new(name, arguments, redirects, is_background))
    }

    pub fn parse_pipe(tokens: &[Token]) -> Result<Pipe> {
        if tokens.len() < 3 {
            Err(Error::NOT_IMPLEMENTED)?
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
