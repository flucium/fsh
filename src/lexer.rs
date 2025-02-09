use crate::{error::*, result::Result, token::Token};

const RESERVED_KEYWORDS: &[&str] = &["null", "true", "false"];

const RESERVED_CHARS: &[char] = &[';', '&', '$', '@', '=', '|', '<', '>', '\'', '"'];

/// Lexer for tokenizing source strings into syntactic tokens.
#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    // length: usize,
    index: usize,
}

impl Lexer {
    /// Creates a new lexer from the given source string.
    ///
    /// # Arguments
    /// - `source`: The input source string to tokenize.
    ///
    /// # Returns
    /// - A `Lexer` instance initialized with the provided source.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into().chars().collect::<Vec<char>>();

        // let length = source.len();

        Self {
            source,
            // length,
            index: 0,
        }
    }

    // pub fn len(&self) -> usize {
    //     self.length
    // }

    // pub fn clear(&mut self) {
    //     self.index = 0;
    //     self.length = 0;
    //     self.source.clear();
    // }

    /// Returns the current character the lexer is processing, if any.
    ///
    /// # Returns
    /// - `Some(&char)`: A reference to the current character.
    /// - `None`: If the lexer has reached the end of the source.
    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

    /// Advances the lexer to the next character.
    ///
    /// This method increments the current index and skips to the next character in the source.
    fn advance(&mut self) {
        self.index += 1;
    }

    /// Reads characters from the source while the given condition is true.
    ///
    /// # Arguments
    /// - `condition`: A closure that returns `true` for characters to include.
    ///
    /// # Returns
    /// - A `String` containing all characters read from the source.
    fn read_while<F>(&mut self, mut condition: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let start_index = self.index;

        while self.current().map_or(false, &mut condition) {
            self.advance();
        }

        self.source[start_index..self.index].iter().collect()
    }

    /// Reads a keyword token from the input.
    ///
    /// This function extracts a contiguous sequence of alphabetic characters
    /// from the current position in the input and attempts to match it to a
    /// known keyword (`true`, `false`, or `null`). If a valid keyword is found,
    /// it returns the corresponding `Token` variant. Otherwise, it restores the
    /// index to its original position and returns an error.
    ///
    /// # Returns
    /// - `Ok(Token::Boolean(true))` if the keyword is `"true"`.
    /// - `Ok(Token::Boolean(false))` if the keyword is `"false"`.
    /// - `Ok(Token::Null)` if the keyword is `"null"`.
    /// - `Err(Error::new(ErrorKind::InvalidSyntax, "invalid keyword"))` if the input does not match a valid keyword.
    fn read_keyword_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| c.is_alphabetic());

        match string.as_str() {
            "true" => Ok(Token::Boolean(true)),

            "false" => Ok(Token::Boolean(false)),

            "null" => Ok(Token::Null),

            _ => {
                self.index = start_index;

                Err(Error::new(ErrorKind::InvalidSyntax, ""))
            }
        }
    }

    /// Reads an unquoted string token from the input.
    ///
    /// This function reads a sequence of non-whitespace characters that are not
    /// reserved symbols. If the extracted string is empty, a number, or a reserved
    /// keyword, it is considered invalid.
    ///
    /// # Returns
    /// - `Ok(Token::String(string))` if a valid string is found.
    /// - `Err(Error::InvalidSyntax)` if the token is invalid.
    fn read_string_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if string.is_empty()
            || string.parse::<isize>().is_ok()
            || RESERVED_KEYWORDS.contains(&string.as_str())
        {
            self.index = start_index;

            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        Ok(Token::String(string))
    }

    /// Reads a quoted string token from the input.
    ///
    /// This function reads a string enclosed in either single (`'`) or double (`"`) quotes.
    /// If the string is not properly enclosed, it returns an error.
    ///
    /// # Returns
    /// - `Ok(Token::String(string))` if a valid quoted string is found.
    /// - `Err(Error::InvalidSyntax)` if there are missing or mismatched quotes.
    fn read_quoted_string_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let quote = self
            .current()
            .ok_or_else(|| Error::new(ErrorKind::InvalidSyntax, ""))?;

        if quote != '\'' && quote != '"' {
            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        self.advance();

        let string = self.read_while(|c| c != quote);

        if self.current() != Some(quote) {
            self.index = start_index;

            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        self.advance();

        Ok(Token::String(string))
    }

    /// Reads an identifier token from the input.
    ///
    /// Identifiers must begin with a `$` character, followed by a sequence of non-whitespace
    /// characters that are not reserved symbols.
    ///
    /// # Returns
    /// - `Ok(Token::Identifier(identifier))` if a valid identifier is found.
    /// - `Err(Error::InvalidSyntax)` if the identifier is empty or `$` is missing.
    fn read_identifier_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        if self.current() != Some('$') {
            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        self.advance();

        let identifier = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if identifier.is_empty() {
            self.index = start_index;

            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        Ok(Token::Identifier(identifier))
    }

    /// Reads a number token from the input.
    ///
    /// This function reads a sequence of numeric characters and converts it into
    /// an integer token.
    ///
    /// # Returns
    /// - `Ok(Token::Number(value))` if a valid number is found.
    /// - `Err(Error::InvalidSyntax)` if the number is malformed.
    fn read_number_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| c.is_numeric());

        string.parse::<isize>().map(Token::Number).map_err(|_| {
            self.index = start_index;
            Error::new(ErrorKind::InvalidSyntax, "")
        })
    }

    /// Reads a file descriptor token from the input.
    ///
    /// File descriptors must begin with `@` followed by a numeric value.
    ///
    /// # Returns
    /// - `Ok(Token::FileDescriptor(fd))` if a valid file descriptor is found.
    /// - `Err(Error::InvalidSyntax)` if the file descriptor is malformed.
    fn read_filedescriptor_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        if self.current() != Some('@') {
            Err(Error::new(ErrorKind::InvalidSyntax, ""))?
        }

        self.advance();

        let string = self.read_while(|c| c.is_numeric());

        string
            .parse::<i32>()
            .map(Token::FileDescriptor)
            .map_err(|_| {
                self.index = start_index;

                Error::new(ErrorKind::InvalidSyntax, "")
            })
    }

    /// Reads the next token from the input.
    ///
    /// This function iterates through characters in the input, skipping whitespace,
    /// and determines the appropriate token type based on the first character.
    /// It delegates to the corresponding `read_*_token` function as needed.
    ///
    /// # Returns
    /// - `Ok(Token::...)` for recognized tokens.
    /// - `Ok(Token::EOF)` if the end of input is reached.
    /// - `Err(Error::InvalidSyntax)` if an invalid token is encountered.
    pub fn next(&mut self) -> Result<Token> {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            return match c {
                ';' => {
                    self.advance();
                    Ok(Token::Semicolon)
                }

                '&' => {
                    self.advance();
                    Ok(Token::Ampersand)
                }

                '|' => {
                    self.advance();
                    Ok(Token::Pipe)
                }

                '=' => {
                    self.advance();
                    Ok(Token::Equal)
                }

                '<' => {
                    self.advance();
                    Ok(Token::LessThan)
                }

                '>' => {
                    self.advance();
                    Ok(Token::GreaterThan)
                }

                '$' => self.read_identifier_token(),

                '@' => self.read_filedescriptor_token(),

                '\'' | '"' => self.read_quoted_string_token(),

                '0'..='9' => self.read_number_token(),

                _ => self
                    .read_keyword_token()
                    .or_else(|_| self.read_string_token()),
            };
        }
        Ok(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserved_keywords() {
        let mut lexer = Lexer::new("null true false");

        assert_eq!(lexer.next(), Ok(Token::Null));

        assert_eq!(lexer.next(), Ok(Token::Boolean(true)));

        assert_eq!(lexer.next(), Ok(Token::Boolean(false)));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_reserved_characters() {
        let mut lexer = Lexer::new("; & | = < >");

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);

        assert_eq!(lexer.next().unwrap(), Token::Ampersand);

        assert_eq!(lexer.next().unwrap(), Token::Pipe);

        assert_eq!(lexer.next().unwrap(), Token::Equal);

        assert_eq!(lexer.next().unwrap(), Token::LessThan);

        assert_eq!(lexer.next().unwrap(), Token::GreaterThan);

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_string_token() {
        let mut lexer = Lexer::new("hello world");

        assert_eq!(lexer.next().unwrap(), Token::String("hello".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::String("world".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_quoted_string_token() {
        let mut lexer = Lexer::new("\"hello world\" 'test string'");

        assert_eq!(
            lexer.next().unwrap(),
            Token::String("hello world".to_string())
        );

        assert_eq!(
            lexer.next().unwrap(),
            Token::String("test string".to_string())
        );

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_identifier_token() {
        let mut lexer = Lexer::new("$identifier1");
        assert_eq!(
            lexer.next().unwrap(),
            Token::Identifier("identifier1".to_string())
        );
        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_number_token() {
        let mut lexer = Lexer::new("123 4567 89");

        assert_eq!(lexer.next().unwrap(), Token::Number(123));

        assert_eq!(lexer.next().unwrap(), Token::Number(4567));

        assert_eq!(lexer.next().unwrap(), Token::Number(89));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_filedescriptor_token() {
        let mut lexer = Lexer::new("@0 @123");

        assert_eq!(lexer.next().unwrap(), Token::FileDescriptor(0));

        assert_eq!(lexer.next().unwrap(), Token::FileDescriptor(123));

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_mixed_tokens() {
        let mut lexer = Lexer::new("$var1 = \"hello\" @3 < 100 & true;");

        assert_eq!(lexer.next().unwrap(), Token::Identifier("var1".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::Equal);

        assert_eq!(lexer.next().unwrap(), Token::String("hello".to_string()));

        assert_eq!(lexer.next().unwrap(), Token::FileDescriptor(3));

        assert_eq!(lexer.next().unwrap(), Token::LessThan);

        assert_eq!(lexer.next().unwrap(), Token::Number(100));

        assert_eq!(lexer.next().unwrap(), Token::Ampersand);

        assert_eq!(lexer.next().unwrap(), Token::Boolean(true));

        assert_eq!(lexer.next().unwrap(), Token::Semicolon);

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("$");

        assert!(lexer.next().is_err());
    }

    #[test]
    fn test_invalid_filedescriptor() {
        let mut lexer = Lexer::new("@notanumber");

        assert!(lexer.next().is_err());
    }

    #[test]
    fn test_invalid_string() {
        let mut lexer = Lexer::new("\"unterminated");

        assert!(lexer.next().is_err());
    }

    #[test]
    fn test_invalid_keyword() {
        let mut lexer = Lexer::new("nulll");

        assert_eq!(lexer.next().unwrap(), Token::String("nulll".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.next().unwrap(), Token::EOF);
    }
}
