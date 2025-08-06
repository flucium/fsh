use std::borrow::Cow;

/// Preprocesses the input shell source by applying normalization steps:
///
/// 1. Removes comments (`# ...`) outside of quoted strings.
/// 2. Removes empty lines.
/// 3. Replaces newlines with semicolons (`;`) to unify statement separation.
/// 4. Collapses consecutive semicolons into a single one.
///
/// This function is typically used to prepare input for parsing by
/// ensuring consistent, linear structure.
///
/// # Arguments
/// - `source`: The raw shell input as a string or convertible type.
///
/// # Returns
/// A cleaned `String` with comments and empty lines removed, and
/// semicolon-delimited logical statements.
pub fn preprocess(source: impl Into<String>) -> String {
    let source = source.into();

    let source = remove_comments(&source);

    let source = remove_empty_line(&source);

    let source = replace_line_with_semicolon(&source);

    let mut chars = source.chars().peekable();
    let mut cleaned = String::with_capacity(source.len());

    while let Some(c) = chars.next() {
        if c == ';' {
            cleaned.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c == ';' {
                    chars.next();
                } else {
                    break;
                }
            }
        } else {
            cleaned.push(c);
        }
    }

    cleaned
    
}

/// Removes comments (`# ...`) from the input source, ignoring those inside quotes.
///
/// A comment starts with `#` and continues until a newline, semicolon, or carriage return,
/// unless it appears within a quoted string (single or double quotes).
///
/// # Arguments
/// - `source`: The input string to clean.
///
/// # Returns
/// A `Cow<str>` containing the result, borrowed if unchanged, or owned if modified.
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


/// Replaces all newline characters (`\n`, `\r\n`) in the input with semicolons (`;`).
///
/// This transforms multi-line input into a single-line semicolon-separated format,
/// suitable for token-based parsing.
///
/// # Arguments
/// - `source`: The input string to process.
///
/// # Returns
/// A `Cow<str>` with all line breaks replaced by semicolons.
#[inline]
fn replace_line_with_semicolon(source: &str) -> Cow<'_, str> {
    source.replace("\r\n", ";").replace("\n", ";").into()
    // Cow::Owned(source.replace("\r\n", ";").replace('\n', ";"))
}

/// Removes all empty or whitespace-only lines from the input string.
///
/// Preserves non-empty lines and retains original order,
/// optionally inserting `\n` between preserved lines.
///
/// # Arguments
/// - `source`: The input string containing multiple lines.
///
/// # Returns
/// A `Cow<str>` with empty lines removed, borrowed if unchanged, or owned if modified.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        assert_eq!(remove_comments("hello # comment"), "hello ");

        assert_eq!(remove_comments("hello\nworld # comment"), "hello\nworld ");

        assert_eq!(
            remove_comments("hello \"# not a comment\" world"),
            "hello \"# not a comment\" world"
        );

        assert_eq!(remove_comments("# full line comment\nhello"), "hello");

        assert_eq!(remove_comments("hello\n# comment\nworld"), "hello\nworld");
    }

    #[test]
    fn test_replace_line_with_semicolon() {
        assert_eq!(replace_line_with_semicolon("hello\nworld"), "hello;world");

        assert_eq!(replace_line_with_semicolon("hello\r\nworld"), "hello;world");

        assert_eq!(
            replace_line_with_semicolon("line1\nline2\nline3"),
            "line1;line2;line3"
        );
    }

    #[test]
    fn test_remove_empty_line() {
        assert_eq!(remove_empty_line("hello\n\nworld"), "hello\nworld");

        assert_eq!(remove_empty_line("\n\nhello\nworld\n\n"), "hello\nworld");

        assert_eq!(remove_empty_line("line1\n\n\nline2"), "line1\nline2");
    }

}

mod benches {
    //
    // e.g. cargo bench --package fsh --lib -- preprocessor::benches::bench_preprocess --exact --show-output
    //
    #[bench]
    fn bench_preprocess(b: &mut test::Bencher) {
        b.iter(|| {
            super::preprocess("a;b;c;");
        });
    }

    #[bench]
    fn bench_remove_comments(b: &mut test::Bencher) {
        b.iter(|| {
            super::remove_comments("hello \"# not a comment\" world");
        });
    }

    #[bench]
    fn bench_replace_line_with_semicolon(b: &mut test::Bencher) {
        b.iter(|| {
            super::replace_line_with_semicolon("hello \"# not a comment\" world");
        });
    }

    #[bench]
    fn bench_remove_empty_line(b: &mut test::Bencher) {
        b.iter(|| {
            super::remove_empty_line("line1\n\n\nline2\n\n");
        });
    }

}
