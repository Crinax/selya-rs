use std::{time::Instant, fs::File};

use domain::Memory;
use memory_exector::MemoryExecutor;
use parser::domain_parsers::selya_parse_from_string;
use pipeline::Pipeline;
use plugin::memory_plugin_string::print_as_utf8;

mod domain;
mod parser;
mod plugin;
mod pipeline;
mod memory_exector;
mod interpreter;

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
        let mut memory_exector = MemoryExecutor::new(*boxed_memory);

        memory_exector.register_plugin(
            "memory-print".into(),
            Box::new(print_as_utf8)
        );
    });

    // let start = Instant::now();
    // let parser_result = selya_parser().parse(
    //     r"
    //         0x000D
    //         0x0048 --> 0x0065 --> 0x006C --> 0x006C
    //         --> 0x006F --> 0x002C --> 0x0020 -->
    //         0x0077 --> 0x006F --> 0x0072 --> 0x006C
    //         --> 0x0064
    //     ",
    // );
    // let end = start.elapsed().as_micros();
    //
    // println!("{:#?}", parser_result);
    // println!("\nElapsed: {}µs", end);
}
