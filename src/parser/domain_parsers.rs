use super::{Parser, map, right, whitespace, match_literal, left, pair, repeats, predicate, any_char, identifier};

pub fn quoted<'a>() -> impl Parser<'a, String> {
    map(
        right(
            match_literal("\""),
            left(
                repeats(predicate(any_char, |c| *c != '"'), 0..),
                match_literal("\""),
            ),
        ),
        |chars| chars.into_iter().collect()
    )
}

pub fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(match_literal("="), quoted()))
}

pub fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    repeats(right(whitespace(1..), attribute_pair()), 0..)
}
