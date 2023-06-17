pub mod domain_parsers;
pub mod tokenizer;

use std::ops::{Bound, RangeBounds};

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

pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(..expected.len()) {
        Some(value) if value == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

pub fn repeats<'a, P, A, R>(parser: P, range: R) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        Bound::Included(value) => *value,
        _ => 0,
    };

    move |input| {
        let mut _input = input;
        let mut result = Vec::new();
        let mut i = 0;

        for _ in 0..start {
            if let Ok((next_value, item)) = parser.parse(_input) {
                i += 1;
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
    repeats(whitespace_char(), repeates_count)
}

pub fn predicate<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }

        Err(input)
    }
}

pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    repeats(parser, 0..)
}

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn match_return_literal<'a>(expected: &'static str) -> impl Parser<'a, String> {
    map(match_literal(expected), |_| expected.into())
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_, _right)| _right)
}

pub fn either<'a, P1, P2, R>(parser1: P1, parser2: P2) -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| match parser1.parse(input) {
        Ok(value) => Ok(value),
        _ => parser2.parse(input),
    }
}
