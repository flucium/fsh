/*
    This is the process that is performed before lexical.
    The input string is preprocessed by the following three steps:
    1. Remove comments.
    2. Remove empty lines.
    3. Replace line with semicolon.
    4. Semicolon cleanup.

    ----------

    Important note:
        Whitespace is not removed during Preprocessing.
            This means that preprocessing does not determine whether or not a whitespace is unnecessary.
            Whitespace is also a character, and its analysis is the role of lexical.
        Preprocessing should only remove or replace Elements (or Token) that are absolutely unnecessary to the extent possible,
            or convert characters to a form that is easier for lexical to handle.

        !Whitespace included in the removal of unnecessary newline codes may be removed by Preprocessing. This 'possibility' is debatable.!
*/

/// Preprocess the input string.
pub(super) fn preprocess(string: &str) -> String {
    let processing = |string: &str| -> String {
        let mut result = string.to_string();

        // P1.
        result = remove_comment(&result);

        // P2.
        result = remove_empty_line(&result);

        // P3.
        result = replace_line_with_semicolon(&result);

        // P4.
        result = semicolon_cleanup(&result);

        result
    };

    processing(string)
}

/// Preprocess P1.
///
/// Remove comments from the input.
#[inline]
fn remove_comment(string: &str) -> String {
    let mut is_comment = false;

    let mut result = String::with_capacity(string.len());

    string.chars().for_each(|c| {
        if c == '#' {
            is_comment = true;
        }

        if is_comment == false {
            result.push(c);
        }

        if c == '\n' || c == '\r' || c == ';' {
            is_comment = false;
        }
    });

    result
}

/// Preprocess P2.
///
/// Remove empty lines from the input.
#[inline]
fn remove_empty_line(string: &str) -> String {
    let mut result = String::with_capacity(string.len());

    for line in string.lines() {
        if line.trim().is_empty() {
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    if result.ends_with('\n') {
        result.remove(result.len() - 1);
    }

    result
}

/// Preprocess P3.
///
/// Replace line with semicolon.
#[inline]
fn replace_line_with_semicolon(string: &str) -> String {
    string.replace("\n", ";")
}

/// Preprocess P4.
///
/// Semicolon cleanup.
///
/// This function is used to remove unnecessary semicolons.
///
/// For example, if the input string is "a;;b", the output string should be "a;b".
#[inline]
fn semicolon_cleanup(string: &str) -> String {
    let mut result = String::with_capacity(string.len());

    let mut is_semicolon = false;

    for c in string.chars() {
        if c == ';' {
            if is_semicolon == false {
                result.push(c);
            }
            is_semicolon = true;
        } else {
            result.push(c);
            is_semicolon = false;
        }
    }

    if result.starts_with(';') {
        result.remove(0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comment() {
        assert_eq!(
            remove_comment("hello world;\n# this is a comment\nhello world;"),
            "hello world;\nhello world;"
        );
    }

    #[test]
    fn test_remove_empty_line() {
        assert_eq!(
            remove_empty_line("hello world;\n\nhello world;"),
            "hello world;\nhello world;"
        );
    }

    #[test]
    fn test_replace_line_with_semicolon() {
        // replace_line_with_semicolon(input: &str) -> String
        assert_eq!(replace_line_with_semicolon("a\nb\nc"), "a;b;c");

        assert_eq!(replace_line_with_semicolon("a\nb\nc\n\n"), "a;b;c;;");
    }

    #[test]
    fn test_semicolon_cleanup() {
        assert_eq!(semicolon_cleanup("a;;b"), "a;b");

        assert_eq!(semicolon_cleanup("a;;b;"), "a;b;");

        assert_eq!(semicolon_cleanup("a;;b;;"), "a;b;");

        assert_eq!(semicolon_cleanup("a;;b;;c"), "a;b;c");

        assert_eq!(semicolon_cleanup("a;;b;;c;"), "a;b;c;");

        assert_eq!(semicolon_cleanup(";a;;b;;c;"), "a;b;c;");

        assert_eq!(semicolon_cleanup("a;;b;;;;;;;;;;;;;c;;;;;;;;"), "a;b;c;");

        assert_eq!(semicolon_cleanup("a;"), "a;");

        assert_eq!(semicolon_cleanup(";a;"), "a;");

        assert_eq!(semicolon_cleanup(";a"), "a");

        assert_eq!(semicolon_cleanup("a"), "a");

        assert_eq!(semicolon_cleanup(";"), "");

        assert_eq!(semicolon_cleanup(""), "");
    }

    #[test]
    fn test_preprocess() {
        assert_eq!(
            preprocess(
                r#"
                # this is a comment
                hello world;
                # this is a comment
                hello world;
                "#,
            ),
            "                                hello world;                                hello world;"
        );

        assert_eq!(
            preprocess(
                r#"
                hello world;

                hello world;
                "#,
            ),
            "                hello world;                hello world;"
        );

        assert_eq!(
            preprocess(
                r#"
                hello world;
                hello world;
                "#,
            ),
            "                hello world;                hello world;"
        );

        assert_eq!(
            preprocess(
                r#"
                hello world;;;;
                hello world;
                "#,
            ),
            "                hello world;                hello world;"
        );
    }
}
