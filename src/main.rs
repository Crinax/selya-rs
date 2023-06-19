use std::{fs::File, time::Instant};

use domain::Memory;
use memory_exector::MemoryExecutor;
use parser::domain_parsers::selya_parse_from_string;
use pipeline::Pipeline;
use plugin::memory_plugin_string::print_as_utf8;

mod domain;
mod interpreter;
mod memory_exector;
mod parser;
mod pipeline;
mod plugin;

fn help() {
    println!("selya {{path}}");
    println!("\t- {{path}} - path to source file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let Ok(file) = File::open(args[1].clone()) else { panic!("Cannot open file"); };
    let mut pipeline = Pipeline::from_file(file);

    pipeline.use_parser(selya_parse_from_string);
    pipeline.use_memory_ctr(Memory::new);
    pipeline.use_memory_executor(|boxed_memory| {
        let mut memory_executor = MemoryExecutor::new(*boxed_memory);

        memory_executor.register_plugin("memory-print".into(), Box::new(print_as_utf8));

        memory_executor.execute(vec!["memory-print".into()]);
    });

    let start = Instant::now();
    pipeline.start();
    let end = start.elapsed().as_micros();

    println!("\nElapsed: {}µs", end);
}
