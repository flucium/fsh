use crate::{
    ast::{expression::*, statement::*},
    error::Error,
    result::Result,
    token::Token,
};

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

fn parse_assignment_value(token: &Token) -> Result<Expression> {
    parse_null(token)
        .or_else(|_| parse_string(token))
        .or_else(|_| parse_boolean(token))
        .or_else(|_| parse_number(token))
        .or_else(|_| parse_file_descriptor(token))
}

pub fn parse_assignment(tokens: &[Token; 3]) -> Result<Assignment> {
    if tokens[1] != Token::Equal {
        Err(Error::NOT_IMPLEMENTED)?;
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
        _ => Err(Error::NOT_IMPLEMENTED)?,
    };
    let right = parse_redirect_right(&tokens[1])?;
    Ok(Redirect::new(operator, left, right))
}

fn parse_normal_redirect(tokens: &[Token; 3]) -> Result<Redirect> {
    let operator = match tokens[1] {
        Token::GreaterThan => RedirectOperator::GreaterThan,
        Token::LessThan => RedirectOperator::LessThan,
        _ => Err(Error::NOT_IMPLEMENTED)?,
    };
    let left = parse_file_descriptor(&tokens[0])?;
    let right = parse_redirect_right(&tokens[2])?;
    Ok(Redirect::new(operator, left, right))
}

pub fn parse_redirect(tokens: &[Token]) -> Result<Redirect> {
    match tokens.len() {
        2 => {
            let arr: &[Token; 2] = tokens.try_into().map_err(|_| Error::NOT_IMPLEMENTED)?;
            parse_abbreviated_redirect(arr)
        }
        3 => {
            let arr: &[Token; 3] = tokens.try_into().map_err(|_| Error::NOT_IMPLEMENTED)?;
            parse_normal_redirect(arr)
        }
        _ => Err(Error::NOT_IMPLEMENTED),
    }
}

fn parse_command_name(token: &Token) -> Result<Expression> {
    parse_string(token)
        .or(parse_identifier(token).or(parse_number(token)))
        .or_else(|_| Err(Error::NOT_IMPLEMENTED))
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
                    parse_number(token)
                        .or(parse_identifier(token).or(parse_string(token)))
                        .or_else(|_| Err(Error::NOT_IMPLEMENTED))?,
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
        pipe.push_back(parse_command(&tokens)?);
    }

    Ok(pipe)
}

fn split(place: &Token, tokens: &[Token]) -> (Vec<Token>, Vec<Token>) {
    if let Some(pos) = tokens.iter().position(|t| t == place) {
        (tokens[..pos].to_vec(), tokens[pos + 1..].to_vec())
    } else {
        (tokens.to_vec(), Vec::new())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::Pipe,
            Token::String("b".to_string()),
        ];

        let (left, right) = split(&Token::Pipe, &tokens);

        assert_eq!(left, vec![Token::String("a".to_string())]);

        assert_eq!(right, vec![Token::String("b".to_string())]);
    }

    #[test]
    fn test_split_no_occurrence() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
        ];

        let (left, right) = split(&Token::Pipe, &tokens);

        assert_eq!(left, tokens);

        assert!(right.is_empty());
    }

    #[test]
    fn test_split_multiple_occurrences() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::Pipe,
            Token::String("b".to_string()),
            Token::Pipe,
            Token::String("c".to_string()),
        ];

        let (left, right) = split(&Token::Pipe, &tokens);

        assert_eq!(left, vec![Token::String("a".to_string())]);

        assert_eq!(
            right,
            vec![
                Token::String("b".to_string()),
                Token::Pipe,
                Token::String("c".to_string())
            ]
        );
    }

    #[test]
    fn test_recursion_split() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::Pipe,
            Token::String("b".to_string()),
            Token::Pipe,
            Token::String("c".to_string()),
        ];

        let result = recursion_split(&Token::Pipe, &tokens);

        assert_eq!(result.len(), 3);

        assert_eq!(result[0], vec![Token::String("a".to_string())]);

        assert_eq!(result[1], vec![Token::String("b".to_string())]);

        assert_eq!(result[2], vec![Token::String("c".to_string())]);
    }

    #[test]
    fn test_recursion_split_no_occurrence() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
        ];

        let result = recursion_split(&Token::Pipe, &tokens);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0], tokens);
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(parse_null(&Token::Null).unwrap(), Expression::Null);

        assert!(parse_null(&Token::String("hello".to_string())).is_err());
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string(&Token::String("hello".to_string())).unwrap(),
            Expression::String("hello".to_string())
        );

        assert!(parse_string(&Token::Number(100)).is_err());
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            parse_identifier(&Token::Identifier("var".to_string())).unwrap(),
            Expression::Identifier("var".to_string())
        );

        assert!(parse_identifier(&Token::String("hello".to_string())).is_err());
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            parse_boolean(&Token::Boolean(true)).unwrap(),
            Expression::Boolean(true)
        );

        assert!(parse_boolean(&Token::Number(1)).is_err());
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse_number(&Token::Number(100)).unwrap(),
            Expression::Number(100)
        );

        assert!(parse_number(&Token::String("100".to_string())).is_err());
    }

    #[test]
    fn test_parse_file_descriptor() {
        assert_eq!(
            parse_file_descriptor(&Token::FileDescriptor(2)).unwrap(),
            Expression::FileDescriptor(2)
        );

        assert!(parse_file_descriptor(&Token::Number(2)).is_err());
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = [
            Token::Identifier("var".to_string()),
            Token::Equal,
            Token::Number(100),
        ];

        let assignment = parse_assignment(&tokens).unwrap();

        assert_eq!(
            assignment.identifier(),
            &Expression::Identifier("var".to_string())
        );

        assert_eq!(assignment.value(), &Expression::Number(100));
    }

    #[test]
    fn test_parse_redirect_abbreviated() {
        let tokens = [
            Token::GreaterThan,
            Token::Identifier("file.txt".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.operator(), &RedirectOperator::GreaterThan);
    }

    #[test]
    fn test_parse_redirect_normal() {
        let tokens = [
            Token::FileDescriptor(1),
            Token::GreaterThan,
            Token::Identifier("output.txt".to_string()),
        ];

        let redirect = parse_redirect(&tokens).unwrap();

        assert_eq!(redirect.operator(), &RedirectOperator::GreaterThan);
    }

    #[test]
    fn test_parse_command() {
        let tokens = [
            Token::String("echo".to_string()),
            Token::String("hello".to_string()),
            Token::FileDescriptor(1),
            Token::GreaterThan,
            Token::String("file.txt".to_string()),
        ];

        let command = parse_command(&tokens).unwrap();

        assert_eq!(command.name(), &Expression::String("echo".to_string()));

        assert_eq!(command.arguments().len(), 1);

        assert_eq!(
            command.arguments()[0],
            Expression::String("hello".to_string())
        );

        assert_eq!(command.redirects().len(), 1);

        assert_eq!(
            command.redirects()[0].operator(),
            &RedirectOperator::GreaterThan
        );

        assert_eq!(
            command.redirects()[0].left(),
            &Expression::FileDescriptor(1)
        );

        assert_eq!(
            command.redirects()[0].right(),
            &Expression::String("file.txt".to_string())
        );
    }

    #[test]
    fn test_parse_pipe() {
        let tokens = [
            Token::String("ls".to_string()),
            Token::Pipe,
            Token::String("cat".to_string()),
            Token::String("-b".to_string()),
        ];

        let mut pipe = parse_pipe(&tokens).unwrap();

        let command = pipe.pop_front().unwrap();
        assert_eq!(command.name(), &Expression::String("ls".to_string()));

        let command = pipe.pop_front().unwrap();
        assert_eq!(command.name(), &Expression::String("cat".to_string()));
        assert_eq!(command.arguments()[0], Expression::String("-b".to_string()));
    }
}
