use crate::token::Token;
// use std::env;

/// Splits a slice of tokens at the first occurrence of the specified token.
///
/// If the token is not found, the entire slice is returned as the left part,
/// and the right part is an empty vector.
///
/// # Arguments
///
/// * `place` - The token at which to split.
/// * `tokens` - The slice of tokens to be split.
///
/// # Returns
///
/// A tuple containing:
/// - `Vec<Token>`: The left part before the `place` token.
/// - `Vec<Token>`: The right part after the `place` token.
pub fn split(place: &Token, tokens: &[Token]) -> (Vec<Token>, Vec<Token>) {
    if tokens.contains(place) == false {
        return (tokens.to_vec(), Vec::with_capacity(0));
    }

    let mut left = Vec::with_capacity(tokens.len());

    let mut right = Vec::with_capacity(tokens.len());

    for (i, token) in tokens.iter().enumerate() {
        if token == place {
            let (l, r) = tokens.split_at(i);

            left = l.to_vec();

            // right = r[1..].to_vec();
            right = r.get(1..).unwrap_or_default().to_vec();

            break;
        }
    }

    (left, right)
}

/// Recursively splits a slice of tokens at every occurrence of the specified token.
///
/// The function repeatedly applies `split` to the right-hand side of each split,
/// collecting the resulting segments into a vector.
///
/// # Arguments
///
/// * `place` - The token at which to split.
/// * `tokens` - The slice of tokens to be split.
///
/// # Returns
///
/// A vector of `Vec<Token>` where each element represents a segment of tokens
/// separated by the `place` token.
pub fn recursion_split(place: &Token, tokens: &[Token]) -> Vec<Vec<Token>> {
    let mut result = Vec::with_capacity(tokens.len());

    let (left, right) = split(place, tokens);

    if right.is_empty() {
        result.push(left);
    } else {
        result.push(left);
        result.append(&mut recursion_split(place, &right));
    }

    result
}

/*
/// Expands a path containing `~` to the user's home directory.
/// 
/// If the path starts with `~`, it is replaced with the value of the `HOME`
/// environment variable. If `HOME` is not set, `/` is used as the fallback.
///
/// # Arguments
///
/// * `path` - The input path, possibly containing `~`.
///
/// # Returns
///
/// A `String` containing the expanded path.
// pub fn expand_home_directory(path: &str) -> String {
//     if path.starts_with("~") {
//         env::var("HOME").unwrap_or_else(|_| String::from("/")) + &path[1..]
//     } else {
//         path.to_string()
//     }
// }
 */

/// Expands a given file path using glob patterns.
///
/// If the given path contains wildcards (`*`, `?`, etc.), it returns all matching paths.
/// If no matches are found, the input path is returned as a single-element vector.
///
/// # Arguments
///
/// * `path` - The input path, potentially containing glob patterns.
///
/// # Returns
///
/// A vector of strings representing the expanded file paths.
pub fn globbing(path: &str) -> Vec<String> {
    if path.is_empty() {
        return Vec::new();
    }

    let mut v = Vec::new();

    glob::glob(&path)
        .map(|paths| {
            paths
                .map(|path| path.unwrap_or_default().to_string_lossy().to_string())
                .collect::<Vec<String>>()
        })
        .map(|paths| {
            if paths.len() > 0 {
                for path in paths {
                    v.push(path);
                }
            } else {
                v.push(path.to_string());
            }
        })
        .unwrap_or_else(|_| v.push(path.to_string()));

    v
}
