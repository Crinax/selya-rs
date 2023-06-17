use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::parser::Parser;
use crate::parser::domain_parsers::SelyaParserResult;
use crate::domain::Memory;

pub struct Pipeline<'a> {
    is_file: bool,
    input: Box<dyn Read + 'a>,
    parser: Option<Box<dyn Parser<'a, SelyaParserResult> + 'a>>,
    memory_ctr: Option<Box<dyn Fn(u16) -> Memory + 'a>>,
    memory_executor: Option<Box<dyn Fn(Box<Memory>) + 'a>>,
}

impl<'a> Pipeline<'a> {
    fn from_file(file: File) -> Box<Pipeline<'a>> {
        Box::new(Pipeline {
            is_file: true,
            input: Box::new(file),
            parser: None,
            memory_ctr: None,
            memory_executor: None,
        })
    }

    pub fn use_parser<T>(&mut self, parser: T)
    where
        T: Parser<'a, SelyaParserResult> + 'a
    {
        self.parser = Some(Box::new(parser))
    }

    pub fn use_memory_ctr<T>(&mut self, memory_ctr: T)
    where
        T: Fn(u16) -> Memory + 'a
    {
        self.memory_ctr = Some(Box::new(memory_ctr))
    }

    pub fn use_memory_executor<T>(&mut self, memory_executor: T)
    where
        T: Fn(Box<Memory>) + 'a
    {
        self.memory_executor = Some(Box::new(memory_executor))
    }

    fn print_pipeline_error(&self, kind: &str, message: &str) {
        println!("[Selya::Pipeline::{}]: {}", kind, message);
    }

    pub fn start(&mut self) {
        if self.parser.is_none() {
            return;
        }

        if self.memory_ctr.is_none() {
            return;
        }

        if self.memory_executor.is_none() {
            return;
        }

        let mut buffer = Rc::new(RefCell::new(String::new()));
        let result = self.input.read_to_string(&mut buffer.borrow_mut());

        match result {
            Ok(_) => {
                let parser = self.parser.as_ref().unwrap();
                let memory_ctr = self.memory_ctr.as_ref().unwrap();
                let memory_executor = self.memory_executor.as_ref().unwrap();
                let parser_result = parser.parse(buffer.borrow().as_str());

                let mut parsing_result: Option<SelyaParserResult> = None;
            },
            Err(_) => {
                self.print_pipeline_error("IoError", "cannot read file");
            },
        }
    }
}
