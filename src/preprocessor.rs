use std::borrow::Cow;

/// Preprocesses a source string by cleaning up its structure.
///
/// This function applies a series of transformations to the input source string:
/// - Removes comments.
/// - Eliminates empty lines.
/// - Replaces line breaks with semicolons.
/// - Collapses consecutive semicolons into a single semicolon.
///
/// # Arguments
/// - `source`: The input source as a type convertible into a `String`.
///
/// # Returns
/// - A `String` containing the cleaned-up source.
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

/// Removes comments from the source string.
///
/// This function strips out all comments starting with a `#` character, except
/// when the `#` appears inside quotes. It preserves all other content, including
/// line breaks and whitespace.
///
/// # Arguments
/// - `source`: A reference to the source string.
///
/// # Returns
/// - A `Cow<str>` containing the source without comments.
///   - If no comments are found, it returns a borrowed reference to the original string.
///   - Otherwise, it returns an owned `String` with comments removed.
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


/// Replaces line breaks with semicolons in the source string.
///
/// This function converts both `\r\n` (Windows-style) and `\n` (Unix-style) line
/// breaks into semicolons to unify line termination.
///
/// # Arguments
/// - `source`: A reference to the source string.
///
/// # Returns
/// - A `Cow<str>` containing the source with line breaks replaced by semicolons.
///   - If no line breaks are found, it returns a borrowed reference to the original string.
///   - Otherwise, it returns an owned `String` with replacements made.
#[inline]
fn replace_line_with_semicolon(source: &str) -> Cow<'_, str> {
    source.replace("\r\n", ";").replace("\n", ";").into()
}


/// Removes empty lines from the source string.
///
/// This function iterates through each line of the source and eliminates lines
/// that consist only of whitespace or are completely empty. Non-empty lines are
/// preserved as is.
///
/// # Arguments
/// - `source`: A reference to the source string.
///
/// # Returns
/// - A `Cow<str>` containing the source without empty lines.
///   - If no empty lines are found, it returns a borrowed reference to the original string.
///   - Otherwise, it returns an owned `String` with empty lines removed.
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
