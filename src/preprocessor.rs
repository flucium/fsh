use std::borrow::Cow;

pub fn preprocess(source: impl Into<String>) -> String {
    let source = source.into();

    let source = remove_comments(&source);

    let source = remove_empty_line(&source);

    let source = replace_line_with_semicolon(&source);

    let mut chars = source.chars().peekable();
    let mut cleaned_result = String::with_capacity(source.len());

    while let Some(c) = chars.next() {
        if c == ';' {
            cleaned_result.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c == ';' {
                    chars.next();
                } else {
                    break;
                }
            }
        } else {
            cleaned_result.push(c);
        }
    }

    cleaned_result
}

#[inline]
fn remove_comments(source: &str) -> Cow<'_, str> {
    let mut result = String::with_capacity(source.len());

    let mut is_comment = false;
    let mut is_quote = false;

    source.chars().for_each(|c| {
        if c == '\'' || c == '"' {
            is_quote = !is_quote;
        }

        if c == '#' && is_quote == false {
            is_comment = true;
        }

        if is_comment == false {
            result.push(c);
        }

        if c == '\r' || c == '\n' || c == ';' {
            is_comment = false;
        }
    });

    if result.len() == source.len() {
        Cow::Borrowed(source)
    } else {
        Cow::Owned(result)
    }
}

#[inline]
fn replace_line_with_semicolon(source: &str) -> Cow<'_, str> {
    source.replace("\r\n", ";").replace("\n", ";").into()
}

#[inline]
fn remove_empty_line(source: &str) -> Cow<'_, str> {
    let mut result = String::with_capacity(source.len());
    let mut is_first_line = true;

    for line in source.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if is_first_line {
            is_first_line = false;
        } else {
            result.push('\n');
        }

        result.push_str(line);
    }

    if result.len() == source.len() {
        Cow::Borrowed(source)
    } else {
        Cow::Owned(result)
    }
}
