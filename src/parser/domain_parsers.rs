use super::{
    any_char, either, map, match_literal, match_return_literal, pair, predicate, repeats, right,
    tokenizer::Token, whitespace, zero_or_more, Parser,
};

fn selya_digit_sign<'a>() -> impl Parser<'a, ()> {
    match_literal("0x")
}

fn selya_number<'a>() -> impl Parser<'a, Token> {
    map(
        right(
            selya_digit_sign(),
            map(
                repeats(predicate(any_char, |c| c.is_ascii_hexdigit()), 1..4),
                |chars| chars.into_iter().collect::<String>(),
            ),
        ),
        |value| Token::Number(u16::from_str_radix(&value, 16).unwrap()),
    )
}

fn selya_shift_right<'a>() -> impl Parser<'a, Token> {
    map(match_literal("-->"), |_| Token::ShiftRight)
}

fn selya_shift_left<'a>() -> impl Parser<'a, Token> {
    map(match_literal("<--"), |_| Token::ShiftLeft)
}

fn selya_add<'a>() -> impl Parser<'a, Token> {
    map(match_return_literal("[+]"), |_| Token::Add)
}

fn selya_add_modulo<'a>() -> impl Parser<'a, Token> {
    map(match_return_literal("[^]"), |_| Token::AddModulo)
}

fn selya_shift_word_right<'a>() -> impl Parser<'a, Token> {
    map(match_return_literal("[>]"), |_| Token::ShiftWordRight)
}

fn selya_shift_word_left<'a>() -> impl Parser<'a, Token> {
    map(match_return_literal("[<]"), |_| Token::ShiftWordLeft)
}

fn selya_left_or_right_shift<'a>() -> impl Parser<'a, Token> {
    either(selya_shift_left(), selya_shift_right())
}

fn selya_left_or_right_word_shift<'a>() -> impl Parser<'a, Token> {
    either(selya_shift_word_left(), selya_shift_word_right())
}

fn selya_one_of_binary_operation<'a>() -> impl Parser<'a, Token> {
    either(selya_add(), selya_add_modulo())
}

fn selya_one_of_nullary_operation<'a>() -> impl Parser<'a, Token> {
    either(
        selya_left_or_right_shift(),
        selya_left_or_right_word_shift(),
    )
}

fn selya_one_of_all_operation<'a>() -> impl Parser<'a, Token> {
    either(
        selya_one_of_nullary_operation(),
        selya_one_of_binary_operation(),
    )
}

fn selya_one_of_token<'a>() -> impl Parser<'a, Token> {
    either(selya_number(), selya_one_of_all_operation())
}

fn with_whitespaces_before<'a, P>(parser: P) -> impl Parser<'a, Token>
where
    P: Parser<'a, Token>,
{
    right(whitespace(0..), parser)
}

pub type SelyaParserResult = (Token, Vec<Token>);

pub fn selya_parser<'a>() -> impl Parser<'a, SelyaParserResult> {
    pair(
        with_whitespaces_before(selya_number()),
        zero_or_more(with_whitespaces_before(selya_one_of_token())),
    )
}
