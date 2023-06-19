use std::fs::File;
use std::io::Read;

use crate::domain::{Memory, MemoryError};
use crate::interpreter::{interprete, InterpreterError};
use crate::parser::domain_parsers::SelyaParserResult;
use crate::parser::tokenizer::UnwrapToken;

pub type ParserInPipeline<'a> = Box<dyn Fn(String) -> Result<SelyaParserResult, String> + 'a>;

pub struct Pipeline<'a> {
    #[allow(dead_code)]
    is_file: bool,
    input: Box<dyn Read + 'a>,
    parser: Option<ParserInPipeline<'a>>,
    memory_ctr: Option<Box<dyn Fn(u16) -> Memory + 'a>>,
    memory_executor: Option<Box<dyn Fn(Box<Memory>) + 'a>>,
}

impl<'a> Pipeline<'a> {
    pub fn from_file(value: File) -> Box<Pipeline<'a>> {
        Box::new(Pipeline {
            is_file: true,
            input: Box::new(value),
            parser: None,
            memory_ctr: None,
            memory_executor: None,
        })
    }

    pub fn use_parser<T>(&mut self, parser: T)
    where
        T: Fn(String) -> Result<SelyaParserResult, String> + 'a,
    {
        self.parser = Some(Box::new(parser))
    }

    pub fn use_memory_ctr<T>(&mut self, memory_ctr: T)
    where
        T: Fn(u16) -> Memory + 'a,
    {
        self.memory_ctr = Some(Box::new(memory_ctr))
    }

    pub fn use_memory_executor<T>(&mut self, memory_executor: T)
    where
        T: Fn(Box<Memory>) + 'a,
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

        let mut buffer = String::new();
        let Ok(_) = self.input.read_to_string(&mut buffer) else {
            self.print_pipeline_error("IoError", "cannot read from file");
            return;
        };

        let parser = self.parser.as_ref().unwrap();
        let memory_ctr = self.memory_ctr.as_ref().unwrap();
        let memory_executor = self.memory_executor.as_ref().unwrap();

        let Ok(parser_result) = parser(buffer) else {
            self.print_pipeline_error("ParserError", "invalid syntax");
            return;
        };

        let mut memory = Box::new(memory_ctr(parser_result.0.unwrap()));

        match interprete(&mut memory, parser_result.1) {
            Ok(_) => (),
            Err(InterpreterError::MemErr(err)) => match err {
                MemoryError::Overflow => {
                    self.print_pipeline_error("Memory::Overflow", "memory overflow");
                    return;
                }
                MemoryError::OutOfRange => {
                    self.print_pipeline_error("Memory::OutOfRange", "memory carriage out of range");
                    return;
                }
            },
            Err(InterpreterError::UsingBinaryAsUnary) => {
                self.print_pipeline_error(
                    "Interpreter::UsingBinaryAsUnary",
                    "cannot using binary operator such [+] and [^] as unary",
                );
                return;
            }
        };

        memory_executor(memory);
    }
}
