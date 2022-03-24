pub mod compiler;
pub mod vm;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    MovePointer(isize),
    Add(isize),
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}
