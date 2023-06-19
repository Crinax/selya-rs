use std::cell::RefCell;
use std::rc::Rc;

pub use crate::domain::Memory;
pub use crate::parser::domain_parsers::SelyaParserResult;
pub use crate::parser::tokenizer::Token;

pub mod memory_plugin_string;

pub trait SelyaPlugin {
    fn execute(&self, memory: Rc<RefCell<Memory>>);
}

impl<T> SelyaPlugin for T
where
    T: Fn(Rc<RefCell<Memory>>),
{
    fn execute(&self, memory: Rc<RefCell<Memory>>) {
        self(memory);
    }
}
