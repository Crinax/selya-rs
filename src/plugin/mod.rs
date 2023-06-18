use std::cell::RefMut;

pub use crate::parser::tokenizer::Token;
pub use crate::parser::domain_parsers::SelyaParserResult;
pub use crate::domain::Memory;

pub mod memory_plugin_string;

pub trait SelyaPlugin {
    fn execute(&self, memory: RefMut<Memory>);
}

impl<T> SelyaPlugin for T
where
    T: Fn(RefMut<Memory>)
{
    fn execute(&self, memory: RefMut<Memory>) {
        
    }
}