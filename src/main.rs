use clap::Parser;
use std::fs;

use brainfuck::{backend::QBE, compiler::Compiler, vm::VM, AppCli};

fn main() {
    let cli = AppCli::parse();
    let input = fs::read_to_string(&cli.file).unwrap();
    let instrs = Compiler::new(input.chars()).compile();
    if cli.execute {
        VM::new(instrs).execute();
    } else if let Some(output) = cli.output {
        fs::write(output, QBE::new(instrs).compile()).unwrap();
    }
}
