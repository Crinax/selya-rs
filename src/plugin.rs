pub use crate::parser::tokenizer::Token;
pub use crate::parser::domain_parsers::SelyaParserResult;
pub use crate::domain::Memory;

use crate::parser::domain_parsers::selya_parser;

pub trait Plugin {
    fn execute()
}