use std::borrow::Cow;

/// Preprocesses the given source string by applying the following steps in order:
///
/// 1. Remove comments (lines starting with `#` outside of quotes).
/// 2. Remove empty lines.
/// 3. Replace line breaks with semicolons.
/// 4. Collapse consecutive semicolons into a single one.
///
/// # Arguments
/// - `source` - Input string to preprocess.
///
/// # Returns
/// - A `String` with comments removed, empty lines skipped,
///   line breaks replaced, and redundant semicolons collapsed.
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
    // Cow::Owned(source.replace("\r\n", ";").replace('\n', ";"))
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

#[cfg(test)]
mod tests {
    //
    // e.g. cargo test --package fsh --lib -- preprocessor::tests --show-output
    //      cargo test --package fsh --lib -- preprocessor:tests::test_remove_comment --exact --show-output
    //      

    use super::*;

    #[test]
    fn test_remove_comment() {
        assert!(remove_comments("").is_empty());

        assert!(remove_comments("#").is_empty());

        assert!(remove_comments("#ls -a").is_empty());

        assert_eq!(remove_comments("\"#ls -a\""), "\"#ls -a\"");

        assert_eq!(remove_comments("ls #-a"), "ls ");

        assert_eq!(remove_comments("#ls;echo Hello"), "echo Hello");

        assert_eq!(remove_comments("#ls\necho Hello"), "echo Hello");
    }

    #[test]
    fn test_replace_line_with_semicolon() {
        assert_eq!(replace_line_with_semicolon("\n"), ";");
        assert_eq!(replace_line_with_semicolon("\n\n\n"), ";;;");

        assert_eq!(replace_line_with_semicolon("\r\n"), ";");
        assert_eq!(replace_line_with_semicolon("\r\n\r\n\r\n"), ";;;");

        assert_ne!(replace_line_with_semicolon("\r"), ";");
        assert_ne!(replace_line_with_semicolon("\n\r"), ";");
    }

    #[test]
    fn test_remove_empty_line() {
        assert!(remove_empty_line("").is_empty());

        assert!(remove_empty_line("  ").is_empty());

        assert_eq!(remove_empty_line("\nHello"), "Hello");

        assert_eq!(remove_empty_line("  \nHello"), "Hello");
    }

    #[test]
    fn test_preprocess() {
        assert_eq!(preprocess(""), "");

        assert_eq!(preprocess(" "), "");

        assert_eq!(preprocess("#hello\nls -a"), "ls -a");

        assert_eq!(preprocess("#hello\n ls -a"), " ls -a");

        assert_eq!(
            preprocess("#hello\nls -a ~; echo '#Hello FSH!' | cat -b;\n\necho Hello."),
            "ls -a ~; echo '#Hello FSH!' | cat -b;echo Hello."
        );
    }
}
