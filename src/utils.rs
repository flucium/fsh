use crate::token::Token;
use std::env;

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

pub fn expand_home_directory(path: &str) -> String {
    if path.starts_with("~") {
        env::var("HOME").unwrap_or_else(|_| String::from("/")) + &path[1..]
    } else {
        path.to_string()
    }
}

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
