pub use crate::parser::tokenizer::Token;
pub use crate::parser::domain_parsers::SelyaParserResult;
pub use crate::domain::Memory;

pub trait Plugin {
    fn make_executor<F>(memory: &Memory) -> F
    where
        F: Fn(&[dyn Plugin]);
}
