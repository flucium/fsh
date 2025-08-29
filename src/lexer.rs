use crate::{error::*, result::Result, token::Token};

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

/// A lexer for tokenizing source input.
#[derive(Debug)]
pub struct Lexer {
    /// The source input as a vector of characters.
    source: Vec<char>,

    /// The current position (cursor) in the input.
    index: usize,
}

impl Lexer {
    /// Creates a new `Lexer` from the given source string.
    ///
    /// # Arguments
    /// - `source` - Input string to tokenize.
    ///
    /// # Returns
    /// - A `Lexer` initialized at the start of the input.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into().chars().collect::<Vec<char>>();

        let index = 0;

        Self { source, index }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }

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

                Err(Error::new(ErrorKind::InvalidSyntax, "unexpected keyword"))
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

            Err(Error::new(ErrorKind::InvalidSyntax, "invalid string token"))?
        }

        Ok(Token::String(string))
    }

    /// Reads a quoted string token enclosed in `'` or `"` characters.
    ///
    /// Returns an error if quotes are unmatched or incomplete.
    fn read_quoted_string_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        let quote = self
            .current()
            .ok_or_else(|| Error::new(ErrorKind::InvalidSyntax, "unterminated quoted string"))?;

        if quote != '\'' && quote != '"' {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid quote character",
            ))?
        }

        self.advance();

        let string = self.read_while(|c| c != quote);

        if self.current() != Some(quote) {
            self.index = start_index;

            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "unterminated quoted string",
            ))?
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
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid identifier start",
            ))?
        }

        self.advance();

        let identifier = self.read_while(|c| !c.is_whitespace() && !RESERVED_CHARS.contains(&c));

        if identifier.is_empty() {
            self.index = start_index;

            Err(Error::new(ErrorKind::InvalidSyntax, "empty identifier"))?
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
            Error::new(ErrorKind::InvalidSyntax, "invalid number literal")
        })
    }

    /// Reads a file descriptor token (e.g., `@1`, `@2`).
    ///
    /// Returns an error if the format is invalid or the number is not an `i32`.
    fn read_filedescriptor_token(&mut self) -> Result<Token> {
        let start_index = self.index;

        if self.current() != Some('@') {
            Err(Error::new(
                ErrorKind::InvalidSyntax,
                "invalid file descriptor start",
            ))?
        }

        self.advance();

        let string = self.read_while(|c| c.is_numeric());

        string
            .parse::<i32>()
            .map(Token::FileDescriptor)
            .map_err(|_| {
                self.index = start_index;

                Error::new(ErrorKind::InvalidSyntax, "invalid file descriptor")
            })
    }

    /// Returns the next token from the input.
    ///
    /// Skips leading whitespace and matches one of the following:
    /// - `;` - semicolon
    /// - `&` - ampersand
    /// - `|` - pipe
    /// - `=` - equal sign
    /// - `<` - less-than
    /// - `>` - greater-than
    /// - `$` - identifier
    /// - `@` - file descriptor
    /// - quoted string
    /// - number
    /// - keyword or string
    ///
    /// # Returns
    /// - `Ok(Token)` with the next token.  
    /// - `Ok(Token::EOF)` if the end of input is reached.  
    /// - `Err(Error)` if tokenization fails.
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next()?;

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
    //
    // e.g. cargo test --package fsh --lib -- lexer::tests --show-output
    //      cargo test --package fsh --lib -- lexer:tests::read_while --exact --show-output
    //

    use super::*;

    #[test]
    fn test_read_while() {
        assert_eq!(Lexer::new("").read_while(|_| false), "");

        assert_eq!(
            Lexer::new("ls -a ./ | cat -b").read_while(|_| true),
            "ls -a ./ | cat -b"
        );

        assert_eq!(
            Lexer::new("ls -a ./ > test.txt; cat < test.txt").read_while(|c| c != ';'),
            "ls -a ./ > test.txt"
        );

        assert_eq!(
            Lexer::new("ls -a | cat -b").read_while(|c| c != ';' && c != '|'),
            "ls -a "
        );

        assert_ne!(
            Lexer::new("ls -a ./ > test.txt; cat < test.txt").read_while(|c| c == ';'),
            "ls -a ./ > test.txt"
        );

        assert_ne!(
            Lexer::new("ls -a | cat -b").read_while(|c| c != ';' || c != '|'),
            "ls -a "
        );
    }

    #[test]
    fn test_read_keyword_token() {
        assert!(Lexer::new("null").read_keyword_token().is_ok());
        assert!(Lexer::new("true").read_keyword_token().is_ok());
        assert!(Lexer::new("false").read_keyword_token().is_ok());

        assert!(Lexer::new("").read_keyword_token().is_err());
        assert!(Lexer::new("abcd").read_keyword_token().is_err());
        assert!(Lexer::new("Null").read_keyword_token().is_err());
        assert!(Lexer::new("True").read_keyword_token().is_err());
        assert!(Lexer::new("False").read_keyword_token().is_err());
    }

    #[test]
    fn test_read_string_token() {
        assert!(Lexer::new("ls -a").read_string_token().is_ok());

        assert_eq!(
            Lexer::new("ls -a").read_string_token(),
            Ok(Token::String("ls".to_string()))
        );

        assert!(Lexer::new("\"ls -a\"").read_string_token().is_err());
        assert!(Lexer::new("0123").read_string_token().is_err());
    }

    #[test]
    fn test_read_quoted_string_token() {
        assert!(Lexer::new("\"echo Hello\"")
            .read_quoted_string_token()
            .is_ok());
        assert!(Lexer::new("'echo Hello'")
            .read_quoted_string_token()
            .is_ok());
        assert!(Lexer::new("\"0123\"").read_quoted_string_token().is_ok());
        assert!(Lexer::new("'0123'").read_quoted_string_token().is_ok());
        assert_eq!(
            Lexer::new("\"#ls -a\"#echo Hello").read_quoted_string_token(),
            Ok(Token::String("#ls -a".to_string()))
        );
        assert_eq!(
            Lexer::new("'#ls -a'#echo Hello").read_quoted_string_token(),
            Ok(Token::String("#ls -a".to_string()))
        );

        assert!(Lexer::new("echo Hello").read_quoted_string_token().is_err());
        assert!(Lexer::new("0123").read_quoted_string_token().is_err());
    }

    #[test]
    fn test_read_identifier_token() {
        assert!(Lexer::new("$A").read_identifier_token().is_ok());
        assert!(Lexer::new("$0123").read_identifier_token().is_ok());
        assert!(Lexer::new("$_").read_identifier_token().is_ok());
        assert!(Lexer::new("$-").read_identifier_token().is_ok());

        assert!(Lexer::new("$\"A\"").read_identifier_token().is_err());
        assert!(Lexer::new("$'A'").read_identifier_token().is_err());
        assert!(Lexer::new("$").read_identifier_token().is_err());
    }

    #[test]
    fn test_read_number_token() {
        assert!(Lexer::new("0123").read_number_token().is_ok());

        assert!(Lexer::new("Hello").read_number_token().is_err());
    }

    #[test]
    fn test_read_filedescriptor_token() {
        assert!(Lexer::new("@0").read_filedescriptor_token().is_ok());
        assert!(Lexer::new("@100").read_filedescriptor_token().is_ok());
        assert_eq!(
            Lexer::new("@0Hello").read_filedescriptor_token(),
            Ok(Token::FileDescriptor(0))
        );

        assert!(Lexer::new("@").read_filedescriptor_token().is_err());
        assert!(Lexer::new("@Hello").read_filedescriptor_token().is_err());
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            Lexer::new("ls -a; echo Hello | rev;echo Hello > test.txt;cat < test.txt;echo \"Hello FSH!\"@1>test.txt;cat @0<test.txt").tokenize(),
            Ok(Vec::from([
                Token::String("ls".to_string()),
                Token::String("-a".to_string()),
                Token::Semicolon,
                Token::String("echo".to_string()),
                Token::String("Hello".to_string()),
                Token::Pipe,
                Token::String("rev".to_string()),
                Token::Semicolon,
                Token::String("echo".to_string()),
                Token::String("Hello".to_string()),
                Token::GreaterThan,
                Token::String("test.txt".to_string()),
                Token::Semicolon,
                Token::String("cat".to_string()),
                Token::LessThan,
                Token::String("test.txt".to_string()),
                Token::Semicolon,
                Token::String("echo".to_string()),
                Token::String("Hello FSH!".to_string()),
                Token::FileDescriptor(1),
                Token::GreaterThan,
                Token::String("test.txt".to_string()),
                Token::Semicolon,
                Token::String("cat".to_string()),
                Token::FileDescriptor(0),
                Token::LessThan,
                Token::String("test.txt".to_string()),
                Token::EOF,
            ]))
        )
    }
}
