use crate::token::Token;

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
