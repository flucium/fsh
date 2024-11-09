use crate::{
    error::{Error, ErrorKind},
    result::Result,
    token::Token,
};

//
// Reserved Keywords
//

const RESERVED_KEYWORD_NULL: &str = "null";

const RESERVED_KEYWORD_TRUE: &str = "true";

const RESERVED_KEYWORD_FALSE: &str = "false";

const RESERVED_KEYWORDS: [&str; 3] = [
    RESERVED_KEYWORD_NULL,
    RESERVED_KEYWORD_TRUE,
    RESERVED_KEYWORD_FALSE,
];

//
// Reserved Characters
//

const RESERVED_CHAR_SEMICOLON: char = ';';

const RESERVED_CHAR_AMPERSAND: char = '&';

const RESERVED_CHAR_DOLLAR: char = '$';

const RESERVED_CHAR_AT: char = '@';

const RESERVED_CHAR_EQUAL: char = '=';

const RESERVED_CHAR_PIPE: char = '|';

const RESERVED_CHAR_LESS_THAN: char = '<';

const RESERVED_CHAR_GREATER_THAN: char = '>';

const RESERVED_CHAR_SINGLE_QUOTE: char = '\'';

const RESERVED_CHAR_DOUBLE_QUOTE: char = '"';

const RESERVED_CHARS: [char; 10] = [
    RESERVED_CHAR_SEMICOLON,
    RESERVED_CHAR_AMPERSAND,
    RESERVED_CHAR_DOLLAR,
    RESERVED_CHAR_AT,
    RESERVED_CHAR_EQUAL,
    RESERVED_CHAR_PIPE,
    RESERVED_CHAR_LESS_THAN,
    RESERVED_CHAR_GREATER_THAN,
    RESERVED_CHAR_SINGLE_QUOTE,
    RESERVED_CHAR_DOUBLE_QUOTE,
];

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    length: usize,
    index: usize,
}

impl Lexer {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into().chars().collect::<Vec<char>>();

        let length = source.len();

        let index = 0;

        Self {
            source,
            length,
            index,
        }
    }

    fn current(&self) -> Option<&char> {
        self.source.get(self.index)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn read_while<F>(&mut self, mut condition: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let start = self.index;

        while let Some(&c) = self.current() {
            if !condition(c) {
                break;
            }

            self.advance();
        }

        self.source
            .get(start..self.index)
            .unwrap_or_default()
            .iter()
            .collect()
    }

    fn read_null_token(&mut self) -> Result<Token> {
        let index = self.index;

        match self.read_while(|c| c.is_alphabetic()).as_str() {
            RESERVED_KEYWORD_NULL => Ok(Token::Null),
            _ => {
                self.index = index;
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "expected 'null' keyword, but encountered an invalid token",
                ))?
            }
        }
    }

    fn read_string_token(&mut self) -> Result<Token> {
        let index = self.index;

        let string = self.read_while(|c| {
            c.is_alphabetic() | c.is_numeric() | !c.is_whitespace() && !RESERVED_CHARS.contains(&c)
        });

        if string.is_empty() {
            self.index = index;
            Err(Error::new(ErrorKind::InvalidSyntax, "string cannot be empty; a valid string requires at least one alphanumeric or symbol character"))?
        }

        if string.parse::<isize>().is_ok() {
            self.index = index;
            Err(Error::new(ErrorKind::InvalidSyntax, "string cannot contain only numeric characters, as it may be mistaken for a number token"))?;
        }

        if RESERVED_KEYWORDS.contains(&string.as_str()) {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "string cannot be a reserved keyword (e.g., 'true', 'false', 'null')",
            ))?;
        }

        Ok(Token::String(string))
    }

    fn read_quoted_string_token(&mut self) -> Result<Token> {
        let index = self.index;

        let (is_single_quote, is_double_quote) = match self.current() {
            Some(&RESERVED_CHAR_SINGLE_QUOTE) => (true, false),
            Some(&RESERVED_CHAR_DOUBLE_QUOTE) => (false, true),
            _ => {
                self.index = index;
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "a quoted string must start with a single or double quotation mark",
                ))?
            }
        };

        if is_single_quote || is_double_quote {
            self.advance();
        } else {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "a quoted string must start with a single or double quotation mark",
            ))?
        }

        let string = self.read_while(|c| {
            (is_single_quote && c != RESERVED_CHAR_SINGLE_QUOTE)
                || (is_double_quote && c != RESERVED_CHAR_DOUBLE_QUOTE)
        });

        if matches!(
            self.current(),
            Some(&RESERVED_CHAR_SINGLE_QUOTE) | Some(&RESERVED_CHAR_DOUBLE_QUOTE)
        ) {
            self.advance();
        } else {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "quoted string must end with the same quotation mark it started with",
            ))?
        }

        Ok(Token::String(string))
    }

    fn read_identifier_token(&mut self) -> Result<Token> {
        let index = self.index;

        if self.current() == Some(&RESERVED_CHAR_DOLLAR) {
            self.advance();
        } else {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "identifier must start with a '$' symbol",
            ))?
        }

        let string = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if string.is_empty() {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "identifier cannot be empty after '$'",
            ))?;
        }

        Ok(Token::Identifier(string))
    }

    fn read_number_token(&mut self) -> Result<Token> {
        let index = self.index;

        let string = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        match string.parse::<isize>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => {
                self.index = index;
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "the token cannot be interpreted as a valid number",
                ))?
            }
        }
    }

    fn read_boolean_token(&mut self) -> Result<Token> {
        let index = self.index;

        match self
            .read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c))
            .as_str()
        {
            RESERVED_KEYWORD_TRUE => Ok(Token::Boolean(true)),
            RESERVED_KEYWORD_FALSE => Ok(Token::Boolean(false)),
            _ => {
                self.index = index;
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "expected 'true' or 'false' for boolean token, but found an invalid token",
                ))?
            }
        }
    }

    fn read_filedescriptor_token(&mut self) -> Result<Token> {
        let index = self.index;

        if self.current() == Some(&RESERVED_CHAR_AT) {
            self.advance();
        } else {
            self.index = index;
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "file descriptor must start with an '@' symbol",
            ))?
        }

        let string = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        match string.parse::<i32>() {
            Ok(n) => Ok(Token::FileDescriptor(n)),
            Err(_) => {
                self.index = index;
                Err(Error::new(
                    ErrorKind::InvalidSyntax,
                    "the token following '@' cannot be interpreted as a valid file descriptor",
                ))?
            }
        }
    }

    pub fn next(&mut self) -> Result<Token> {
        while let Some(&c) = self.current() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            match c {
                RESERVED_CHAR_SEMICOLON => {
                    self.advance();
                    return Ok(Token::Semicolon);
                }

                RESERVED_CHAR_AMPERSAND => {
                    self.advance();
                    return Ok(Token::Ampersand);
                }

                RESERVED_CHAR_PIPE => {
                    self.advance();
                    return Ok(Token::Pipe);
                }

                RESERVED_CHAR_EQUAL => {
                    self.advance();
                    return Ok(Token::Equal);
                }

                RESERVED_CHAR_LESS_THAN => {
                    self.advance();
                    return Ok(Token::LessThan);
                }

                RESERVED_CHAR_GREATER_THAN => {
                    self.advance();
                    return Ok(Token::GreaterThan);
                }

                RESERVED_CHAR_DOLLAR => return self.read_identifier_token(),

                RESERVED_CHAR_AT => return self.read_filedescriptor_token(),

                RESERVED_CHAR_SINGLE_QUOTE | RESERVED_CHAR_DOUBLE_QUOTE => {
                    return self.read_quoted_string_token()
                }

                '0'..='9' => return self.read_number_token(),

                _ => {
                    // return self.read_number_token().or(self
                    //     .read_boolean_token()
                    //     .or(self.read_null_token().or(self.read_string_token())));

                    return self
                        .read_number_token()
                        .or(self.read_boolean_token())
                        .or(self.read_null_token())
                        .or(self.read_string_token());
                }
            }
        }

        if self.index >= self.length {
            return Ok(Token::EOF);
        } else {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "unrecognized or invalid token encountered",
            ))?
        }
    }
}
