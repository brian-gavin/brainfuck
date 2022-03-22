use std::{env, fs};

use brainfuck::{Compiler, VM};

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap_or_default()).unwrap();
    let instrs = Compiler::new(input.chars()).compile();
    VM::new(instrs).execute();
}
