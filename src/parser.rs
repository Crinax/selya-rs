pub mod domain_parsers;

use std::ops::{RangeBounds, Bound};

pub type ParserResult<'a, Output> = Result<(&'a str, Output), &'a str>;
pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParserResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParserResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParserResult<'a, Output> {
        self(input)
    }
}

pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()>
{
    move |input: &'a str| match input.get(..expected.len()) {
        Some(value) if value == expected => {
            Ok((&input[expected.len()..], ()))
        },
        _ => Err(input)
    }
}

pub fn identifier<'a>(input: &'a str) -> ParserResult<'a, String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(value) if value.is_alphabetic() => matched.push(value),
        _ => return Err(input)
    };

    while let Some(value) = chars.next() {
        if !(value.is_alphanumeric() || value == '-') {
            break;
        }

        matched.push(value);
    }

    let next_index = matched.len();
    
    Ok((&input[next_index..], matched))
}

// Доделотьб
pub fn repeats<'a, P, A, R>(parser: P, range: R) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
    R: RangeBounds<usize>
{
    let start = match range.start_bound() {
        Bound::Included(value) => *value,
        _ => 0,
    };

    move |input| {
        let mut _input = input;
        let mut result = Vec::new();
        let mut i = 0;

        for it in 0..start {
            if let Ok((next_value, item)) = parser.parse(_input) {
                i = it;
                _input = next_value;
                result.push(item);
            } else {
                return Err(_input);
            }
        }

        while range.contains(&i) {
            if let Ok((next_value, item)) = parser.parse(_input) {
                _input = next_value;
                result.push(item);
            } else {
                break;
            }
        }

        Ok((_input, result))
    }
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A> + Copy
{
    repeats(parser, 1..)
}

pub fn any_char(input: &str) -> ParserResult<char> {
    match input.chars().next() {
        Some(value) => Ok((&input[value.len_utf8()..], value)),
        _ => Err(input),
    }
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    predicate(any_char, |c| c.is_whitespace())
}

pub fn whitespace<'a>(repeates_count: impl RangeBounds<usize>) -> impl Parser<'a, Vec<char>> {
    repeats(predicate(any_char, |c| c.is_whitespace()), repeates_count)
}

pub fn predicate<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value))
            }
        }

        Err(input)
    }
}

pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>
{
    repeats(parser, 0..)
}

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2)
    -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F)
    -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2)
    -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, _)| _left)
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2)
    -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_, _right)| _right)
}
