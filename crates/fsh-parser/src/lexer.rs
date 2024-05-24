use crate::preprocess::preprocess;

use super::token::Token;

const RESERVED_SYMBOLS: [char; 11] = [
    RESERVED_SYMBOL_SEMICOLON,
    RESERVED_SYMBOL_EQUAL,
    RESERVED_SYMBOL_BACKSLASH,
    RESERVED_SYMBOL_SINGLE_QUOTE,
    RESERVED_SYMBOL_DOUBLE_QUOTE,
    RESERVED_SYMBOL_AMPERSAND,
    RESERVED_SYMBOL_DOLLAR,
    RESERVED_SYMBOL_AT,
    RESERVED_SYMBOL_PIPE,
    RESERVED_SYMBOL_GT,
    RESERVED_SYMBOL_LT,
];

const RESERVED_SYMBOL_SEMICOLON: char = ';';

const RESERVED_SYMBOL_EQUAL: char = '=';

const RESERVED_SYMBOL_BACKSLASH: char = '\\';

const RESERVED_SYMBOL_SINGLE_QUOTE: char = '\'';

const RESERVED_SYMBOL_DOUBLE_QUOTE: char = '"';

const RESERVED_SYMBOL_AMPERSAND: char = '&';

const RESERVED_SYMBOL_DOLLAR: char = '$';

const RESERVED_SYMBOL_AT: char = '@';

const RESERVED_SYMBOL_PIPE: char = '|';

const RESERVED_SYMBOL_GT: char = '>';

const RESERVED_SYMBOL_LT: char = '<';

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    /// Create a new lexer.
    pub fn new(source: &str) -> Self {
        let source = preprocess(source);

        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    /// Get the current character.
    fn current_char(&self) -> Option<&char> {
        self.source.get(self.position)
    }

    /// Get the peek next character.
    fn peek_char(&self) -> Option<&char> {
        self.source.get(self.position + 1)
    }

    /// Advance the position.
    fn advance(&mut self) {
        self.position += 1;
    }

    // Retreat the position.
    // fn retreat(&mut self) {
    //     self.position -= 1;
    // }

    /// Read while the condition is true.
    ///
    /// # Arguments
    /// * `f` - A closure that takes a character and returns a boolean.
    ///
    /// # Returns
    /// A tuple of a vector of characters and an optional reference to a character.
    ///
    /// The vector of characters is the characters that satisfy the condition.
    ///
    /// The optional reference to a character is the next character that does not satisfy the condition. (Position has not moved to the char here.)
    fn read_while<F>(&mut self, f: F) -> (Vec<char>, Option<&char>)
    where
        F: Fn(char) -> bool,
    {
        let mut result = Vec::new();

        while let Some(&c) = self.source.get(self.position) {
            if f(c) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let next = self.source.get(self.position);

        (result, next)
    }

    /// Read a string.
    fn read_string(&mut self) -> std::result::Result<Option<String>, String> {
        let start_position = self.position;

        let (is_double_quote, is_single_quote) = match self.current_char() {
            None => return Ok(None),
            Some(&RESERVED_SYMBOL_DOUBLE_QUOTE) => (true, false),
            Some(&RESERVED_SYMBOL_SINGLE_QUOTE) => (false, true),
            Some(_) => (false, false),
        };

        if is_double_quote || is_single_quote {
            self.advance();
        }

        let (string, end_char) = self.read_while(|c| {
            if is_double_quote {
                c != RESERVED_SYMBOL_DOUBLE_QUOTE
            } else if is_single_quote {
                c != RESERVED_SYMBOL_SINGLE_QUOTE
            } else {
                !RESERVED_SYMBOLS.contains(&c) && !c.is_whitespace()
            }
        });

        if is_double_quote && end_char != Some(&RESERVED_SYMBOL_DOUBLE_QUOTE) {
            self.position = start_position;

            Err("double quote error".to_string())?
        } else if is_single_quote && end_char != Some(&RESERVED_SYMBOL_SINGLE_QUOTE) {
            self.position = start_position;

            Err("single quote error".to_string())?
        } else if !is_double_quote && !is_single_quote && string.is_empty() {
            self.position = start_position;

            return Ok(None);
        } else {
            if is_double_quote || is_single_quote {
                self.advance();
            }
        }

        Ok(Some(string.into_iter().collect()))
    }

    /// Read a number.
    fn read_number(&mut self) -> std::result::Result<Option<usize>, String> {
        match self.current_char() {
            Some(c) => {
                if !c.is_digit(10) {
                    Err("invalid number".to_string())?
                }
            }
            None => return Ok(None),
        }

        let start_position = self.position;

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !RESERVED_SYMBOLS.contains(&c));

        match string.into_iter().collect::<String>().parse::<usize>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => {
                self.position = start_position;
                Err("invalid number".to_string())
            }
        }
    }

    /// Read an ident.
    fn read_ident(&mut self) -> std::result::Result<Option<String>, String> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Ok(None);
        }

        let start_position = self.position;

        if current_char == Some(&RESERVED_SYMBOL_DOLLAR) {
            self.advance();
        } else {
            self.position = start_position;
            Err("invalid ident".to_string())?
        }

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !RESERVED_SYMBOLS.contains(&c));

        if string.is_empty() {
            self.position = start_position;
            Err("invalid ident".to_string())?
        }

        if let Some(ch) = string.first() {
            if !ch.is_alphabetic() {
                self.position = start_position;
                Err("invalid ident".to_string())?
            }
        } else {
            self.position = start_position;
            Err("invalid ident".to_string())?
        }

        if let Some(c) = string.last() {
            if !c.is_alphanumeric() {
                self.position = start_position;
                Err("invalid ident".to_string())?
            }
        } else {
            self.position = start_position;
            Err("invalid ident".to_string())?
        }

        Ok(Some(string.into_iter().collect()))
    }

    /// Read a file descriptor.
    fn read_fd(&mut self) -> std::result::Result<Option<usize>, String> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Ok(None);
        }

        let start_position = self.position;

        if current_char == Some(&RESERVED_SYMBOL_AT) {
            self.advance();
        } else {
            self.position = start_position;
            Err("invalid file descriptor".to_string())?
        }

        let (string, _) = self.read_while(|c| !c.is_whitespace() && !RESERVED_SYMBOLS.contains(&c));

        match string.into_iter().collect::<String>().parse::<usize>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => {
                self.position = start_position;
                Err("invalid file descriptor".to_string())
            }
        }
    }

    /// Read a token.
    fn read(&mut self) -> fsh_common::Result<Token> {
        let mut token = Token::EOF;

        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            match ch {
                &RESERVED_SYMBOL_SEMICOLON => {
                    token = Token::Semicolon;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_EQUAL => {
                    token = Token::Assign;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_AMPERSAND => {
                    token = Token::Ampersand;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_PIPE => {
                    token = Token::Pipe;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_GT => {
                    token = Token::Gt;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_LT => {
                    token = Token::Lt;
                    self.advance();
                    break;
                }

                &RESERVED_SYMBOL_AT => {
                    match self.read_fd() {
                        Ok(Some(fd)) => token = Token::FD(fd as i32),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.peek_char() {
                            Some(ch) => {
                                if ch.is_whitespace() {
                                    token = Token::String(RESERVED_SYMBOL_AT.to_string());
                                    self.advance();
                                } else {
                                    Err(fsh_common::Error::new(
                                        fsh_common::ErrorKind::LexerError,
                                        &err,
                                    ))?;
                                }
                            }
                            None => {
                                token = Token::String(RESERVED_SYMBOL_AT.to_string());
                                self.advance();
                            }
                        },
                    }

                    break;
                }

                &RESERVED_SYMBOL_DOLLAR => {
                    match self.read_ident() {
                        Ok(Some(ident)) => token = Token::Ident(ident),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.peek_char() {
                            Some(ch) => {
                                if ch.is_whitespace() {
                                    token = Token::String(RESERVED_SYMBOL_DOLLAR.to_string());
                                    self.advance();
                                } else {
                                    Err(fsh_common::Error::new(
                                        fsh_common::ErrorKind::LexerError,
                                        &err,
                                    ))?;
                                }
                            }
                            None => {
                                token = Token::String(RESERVED_SYMBOL_DOLLAR.to_string());
                                self.advance();
                            }
                        },
                    }

                    break;
                }

                &RESERVED_SYMBOL_DOUBLE_QUOTE | &RESERVED_SYMBOL_SINGLE_QUOTE => {
                    match self.read_string() {
                        Ok(Some(string)) => token = Token::String(string),
                        Ok(None) => token = Token::EOF,
                        Err(err) => Err(fsh_common::Error::new(
                            fsh_common::ErrorKind::LexerError,
                            &err,
                        ))?,
                    }

                    break;
                }

                '0'..='9' => {
                    match self.read_number() {
                        Ok(Some(number)) => token = Token::Number(number),
                        Ok(None) => token = Token::EOF,
                        Err(err) => match self.read_string() {
                            Ok(Some(string)) => token = Token::String(string),
                            Ok(None) => token = Token::EOF,
                            Err(_) => Err(fsh_common::Error::new(
                                fsh_common::ErrorKind::LexerError,
                                &err,
                            ))?,
                        },
                    }

                    break;
                }

                _ => {
                    match self.read_string() {
                        Ok(Some(string)) => token = Token::String(string),
                        Ok(None) => token = Token::EOF,
                        Err(err) => {
                            if self.peek_char().is_some() {
                                Err(fsh_common::Error::new(
                                    fsh_common::ErrorKind::LexerError,
                                    &err,
                                ))?;
                            } else {
                                token = Token::EOF;
                            }
                        }
                    }

                    break;
                }
            }
        }

        Ok(token)
    }

    /// Tokenize the source.
    pub fn tokenize(&mut self) -> fsh_common::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.read()?;

            if token == Token::Semicolon {
                if tokens.last() == Some(&Token::Semicolon) {
                    continue;
                }
            }

            tokens.push(token);

            if tokens.last() == Some(&Token::EOF) {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string() {
        assert_eq!(
            Lexer::new("echo hello world")
                .read_string()
                .unwrap()
                .unwrap(),
            "echo"
        );

        assert_eq!(
            Lexer::new("echo \"hello world\"")
                .read_string()
                .unwrap()
                .unwrap(),
            "echo"
        );

        assert_eq!(
            Lexer::new("\"echo hello world\"")
                .read_string()
                .unwrap()
                .unwrap(),
            "echo hello world"
        );

        assert_eq!(
            Lexer::new("'echo hello world'")
                .read_string()
                .unwrap()
                .unwrap(),
            "echo hello world"
        );
    }

    #[test]
    fn test_read_number() {
        assert_eq!(Lexer::new("123").read_number().unwrap().unwrap(), 123);

        assert_eq!(Lexer::new("123 ").read_number().unwrap().unwrap(), 123);

        assert_eq!(Lexer::new("123    ").read_number().unwrap().unwrap(), 123);
    }

    #[test]
    fn test_read_number_error() {
        assert!(Lexer::new("abc").read_number().is_err());

        assert!(Lexer::new("a123").read_number().is_err());

        assert!(Lexer::new("123a").read_number().is_err());

        assert!(Lexer::new(" 123").read_number().is_err());

        assert!(Lexer::new(" 123 ").read_number().is_err());
    }

    #[test]
    fn test_read_ident() {
        assert_eq!(Lexer::new("$abc").read_ident().unwrap().unwrap(), "abc");

        assert_eq!(Lexer::new("$abc1").read_ident().unwrap().unwrap(), "abc1");

        assert_eq!(Lexer::new("$a;bc").read_ident().unwrap().unwrap(), "a");

        assert_eq!(Lexer::new("$abc ").read_ident().unwrap().unwrap(), "abc");

        assert_eq!(Lexer::new("$abc    ").read_ident().unwrap().unwrap(), "abc");
    }

    #[test]
    fn test_read_ident_error() {
        assert!(Lexer::new("abc").read_ident().is_err());

        assert!(Lexer::new("123").read_ident().is_err());

        assert!(Lexer::new(" $abc").read_ident().is_err());

        assert!(Lexer::new(" $abc ").read_ident().is_err());

        assert!(Lexer::new("$123").read_ident().is_err());

        assert!(Lexer::new("$!").read_ident().is_err());

        assert!(Lexer::new("$\"hello fsh\"").read_ident().is_err());
    }

    #[test]
    fn test_read_fd() {
        assert_eq!(Lexer::new("@1").read_fd().unwrap().unwrap(), 1);

        assert_eq!(Lexer::new("@1 ").read_fd().unwrap().unwrap(), 1);

        assert_eq!(Lexer::new("@1    ").read_fd().unwrap().unwrap(), 1);
    }

    #[test]
    fn test_read_fd_error() {
        assert!(Lexer::new("1").read_fd().is_err());

        assert!(Lexer::new("a1").read_fd().is_err());

        assert!(Lexer::new("1a").read_fd().is_err());

        assert!(Lexer::new(" @1").read_fd().is_err());

        assert!(Lexer::new(" @1 ").read_fd().is_err());

        assert!(Lexer::new("@1a").read_fd().is_err());

        assert!(Lexer::new("@!").read_fd().is_err());

        assert!(Lexer::new("@\"1 2\"").read_fd().is_err());
    }

    #[test]
    fn test_read() {
        let mut lexer =
            Lexer::new("echo hello world;echo 'hello world';ls ./ @1 > file.txt;ls $P | cat -b");

        assert_eq!(lexer.read().unwrap(), Token::String("echo".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::String("hello".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::String("world".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::Semicolon);

        assert_eq!(lexer.read().unwrap(), Token::String("echo".to_string()));

        assert_eq!(
            lexer.read().unwrap(),
            Token::String("hello world".to_string())
        );

        assert_eq!(lexer.read().unwrap(), Token::Semicolon);

        assert_eq!(lexer.read().unwrap(), Token::String("ls".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::String("./".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::FD(1));

        assert_eq!(lexer.read().unwrap(), Token::Gt);

        assert_eq!(lexer.read().unwrap(), Token::String("file.txt".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::Semicolon);

        assert_eq!(lexer.read().unwrap(), Token::String("ls".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::Ident("P".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::Pipe);

        assert_eq!(lexer.read().unwrap(), Token::String("cat".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::String("-b".to_string()));

        assert_eq!(lexer.read().unwrap(), Token::EOF);
    }

    #[test]
    fn test_tokenize() {
        let mut lexer =
            Lexer::new("echo hello world;echo 'hello world';ls ./ @1 > file.txt;ls $P | cat -b");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 19);

        assert_eq!(tokens[0], Token::String("echo".to_string()));

        assert_eq!(tokens[1], Token::String("hello".to_string()));

        assert_eq!(tokens[2], Token::String("world".to_string()));

        assert_eq!(tokens[3], Token::Semicolon);

        assert_eq!(tokens[4], Token::String("echo".to_string()));

        assert_eq!(tokens[5], Token::String("hello world".to_string()));

        assert_eq!(tokens[6], Token::Semicolon);

        assert_eq!(tokens[7], Token::String("ls".to_string()));

        assert_eq!(tokens[8], Token::String("./".to_string()));

        assert_eq!(tokens[9], Token::FD(1));

        assert_eq!(tokens[10], Token::Gt);

        assert_eq!(tokens[11], Token::String("file.txt".to_string()));

        assert_eq!(tokens[12], Token::Semicolon);

        assert_eq!(tokens[13], Token::String("ls".to_string()));

        assert_eq!(tokens[14], Token::Ident("P".to_string()));

        assert_eq!(tokens[15], Token::Pipe);

        assert_eq!(tokens[16], Token::String("cat".to_string()));

        assert_eq!(tokens[17], Token::String("-b".to_string()));

        assert_eq!(tokens[18], Token::EOF);
    }

}
