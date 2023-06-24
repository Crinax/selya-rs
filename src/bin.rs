use std::{fs::File, time::Instant};

use selya::{
    Memory,
    MemoryExecutor,
    selya_parse_from_string,
    Pipeline,
    memory_plugin_string::print_as_utf8
};
use clap::Parser;


/// SELYA - Special Esoteric Language for Young and Adult
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to source file
    file: String,

    /// Don't print execution time after execution
    #[arg(short, long)]
    quit: bool,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = File::open(args.file) else {
        println!("[Selya::FileReadError]: cannot open file");
        return;
    };
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

    if !args.quit {
        println!("\nElapsed: {}µs", end);
    }
}
