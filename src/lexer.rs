use crate::{error::Error, result::Result, token::Token};

/*
    0 null
    1 true
    2 false
*/
const RESERVED_KEYWORDS: &[&str] = &["null", "true", "false"];

/*
    0 ;
    1 &
    2 $
    3 @
    4 =
    5 |
    6 <
    7 >
    8 '
    9 "
*/
const RESERVED_CHARS: &[char] = &[';', '&', '$', '@', '=', '|', '<', '>', '\'', '"'];

/// A simple character-based lexer for tokenizing FSH input.
///
/// The lexer operates over a character stream and produces a sequence
/// of `Token` values representing syntactic elements like strings,
/// numbers, symbols, and identifiers.
#[derive(Debug)]
pub struct Lexer {
    /// The source input as a vector of characters.
    source: Vec<char>,

    /// The current position (cursor) in the input.
    index: usize,
}

impl Lexer {
    /// Creates a new `Lexer` from a string-like input.
    ///
    /// # Arguments
    /// - `source`: The input to tokenize.
    ///
    /// # Returns
    /// A `Lexer` instance ready to produce tokens from the input.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into().chars().collect::<Vec<char>>();

        let index = 0;

        Self { source, index }
    }

    /// Returns the current character under the cursor, if any.
    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

    /// Advances the cursor by one character.
    fn advance(&mut self) {
        self.index += 1;
    }

    /// Reads characters while the given predicate returns `true`.
    ///
    /// Returns the collected substring and stops when the condition fails.
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

    /// Attempts to read a keyword token: `true`, `false`, or `null`.
    ///
    /// Resets the cursor and returns an error if no known keyword is found.
    fn read_keyword_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| c.is_alphabetic());

        // null
        // true
        // false
        // _ => Err
        match string {
            val if val == RESERVED_KEYWORDS[0] => Ok(Token::Null),

            val if val == RESERVED_KEYWORDS[1] => Ok(Token::Boolean(true)),

            val if val == RESERVED_KEYWORDS[2] => Ok(Token::Boolean(false)),

            _ => {
                self.index = start_index;

                Err(Error::NOT_IMPLEMENTED)
            }
        }
    }

    /// Reads a non-quoted string token until whitespace or reserved character.
    ///
    /// Fails if the token is empty, numeric, or a reserved keyword.
    fn read_string_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if string.is_empty()
            || string.parse::<isize>().is_ok()
            || RESERVED_KEYWORDS.contains(&string.as_str())
        {
            self.index = start_index;

            Err(Error::NOT_IMPLEMENTED)?
        }

        Ok(Token::String(string))
    }

    /// Reads a quoted string token enclosed in `'` or `"` characters.
    ///
    /// Returns an error if quotes are unmatched or incomplete.
    fn read_quoted_string_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let quote = self.current().ok_or_else(|| Error::NOT_IMPLEMENTED)?;

        if quote != '\'' && quote != '"' {
            Err(Error::NOT_IMPLEMENTED)?
        }

        self.advance();

        let string = self.read_while(|c| c != quote);

        if self.current() != Some(quote) {
            self.index = start_index;

            Err(Error::NOT_IMPLEMENTED)?
        }

        self.advance();

        Ok(Token::String(string))
    }

    /// Reads a shell variable identifier (variable key) token (e.g., `$HOME`).
    ///
    /// Returns an error if the identifier is empty or malformed.
    fn read_identifier_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        if self.current() != Some('$') {
            Err(Error::NOT_IMPLEMENTED)?
        }

        self.advance();

        let identifier = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if identifier.is_empty() {
            self.index = start_index;

            Err(Error::NOT_IMPLEMENTED)?
        }

        Ok(Token::Identifier(identifier))
    }

    /// Reads an integer literal token.
    ///
    /// Returns an error if the value cannot be parsed as `isize`.
    fn read_number_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let string = self.read_while(|c| c.is_numeric());

        string.parse::<isize>().map(Token::Number).map_err(|_| {
            self.index = start_index;
            Error::NOT_IMPLEMENTED
        })
    }

    /// Reads a file descriptor token (e.g., `@1`, `@2`).
    ///
    /// Returns an error if the format is invalid or the number is not an `i32`.
    fn read_filedescriptor_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        if self.current() != Some('@') {
            Err(Error::NOT_IMPLEMENTED)?
        }

        self.advance();

        let string = self.read_while(|c| c.is_numeric());

        string
            .parse::<i32>()
            .map(Token::FileDescriptor)
            .map_err(|_| {
                self.index = start_index;

                Error::NOT_IMPLEMENTED
            })
    }

    /// Returns the next token from the input stream.
    ///
    /// Skips leading whitespace and dispatches based on the next character.
    ///
    /// Supported token types include:
    /// - Punctuation: `;`, `&`, `|`, `=`, `<`, `>`
    /// - Identifiers: `$name`
    /// - File descriptors: `@1`
    /// - Quoted strings: `"..."`, `'...'`
    /// - Keywords: `true`, `false`, `null`
    /// - Numbers: `123`
    /// - Strings: non-reserved, non-numeric, unquoted words
    ///
    /// Returns `Token::EOF` when input is fully consumed.
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
