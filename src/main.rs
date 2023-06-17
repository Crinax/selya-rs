use std::time::Instant;

use crate::parser::{
    any_char, domain_parsers::selya_parser, match_return_literal, pair, repeats, right, whitespace,
    zero_or_more, Parser,
};

mod domain;
mod parser;

fn main() {
    let start = Instant::now();
    let parser_result = selya_parser().parse(
        r"
            0x000D
            0x0048 --> 0x0065 --> 0x006C --> 0x006C
            --> 0x006F --> 0x002C --> 0x0020 -->
            0x0077 --> 0x006F --> 0x0072 --> 0x006C
            --> 0x0064
        ",
    );
    let end = start.elapsed().as_micros();

    println!("{:#?}", parser_result);
    println!("\nElapsed: {}µs", end);
}
