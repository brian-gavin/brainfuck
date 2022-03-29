use clap::Parser;

pub mod backend;
pub mod compiler;
pub mod vm;

/// Brainfuck VM and Compiler.
#[derive(Parser, Debug)]
pub struct AppCli {
    /// Specifies that the input file should be executed with the VM.
    #[clap(short, long)]
    pub execute: bool,
    /// Compile the input file into QBE `.ssa` file <FILE>
    #[clap(short, long, value_name = "FILE")]
    pub output: Option<String>,
    /// The file to execute or compile.
    pub file: String,
}

const NUM_CELLS: usize = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    MovePointer(isize),
    Add(isize),
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}
