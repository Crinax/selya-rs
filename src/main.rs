
mod domain;
mod parser;

use crate::parser::{domain_parsers::{attributes, attribute_pair}, Parser};

fn main() {
    println!(
        "{:?}",
        attributes().parse(" f=\"dom\" b=\"dom\"")
    );

    println!("{:?}", attribute_pair().parse("f=\"123\""))
}
