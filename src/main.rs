mod domain;
mod parser;

use crate::parser::{match_literal, pair, repeats, identifier, right, Parser, whitespace_char};

// Надо подумать над реализацией repeats для RangeBounds без выбрасывания ошибки
// Либо выбрасывать ошибку уже в самом парсинге
fn main() {
    println!("aaaab: {:?}", repeats(pair(identifier, whitespace_char()), 0..).parse("*(^&(&^))"));
    println!("aaaab: {:?}", repeats(pair(identifier, whitespace_char()), 2..).parse("aaaab"));
    println!("{:?}", right(match_literal("<"), identifier).parse("<xuy />"));
}
