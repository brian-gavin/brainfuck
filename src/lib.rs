use std::{
    collections::HashMap,
    io::{self, ErrorKind, Read},
};

pub struct Compiler<I>
where
    I: Iterator<Item = char>,
{
    input: I,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    MovePointer(isize),
    Add(isize),
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}

impl<I> Compiler<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Compiler<I> {
        Compiler { input }
    }

    pub fn compile(mut self) -> Vec<Instruction> {
        use Instruction::*;
        let mut v = vec![];
        let mut check_jumps = vec![];
        while let Some(c) = self.input.next() {
            match c {
                '>' => v.push(MovePointer(1)),
                '<' => v.push(MovePointer(-1)),
                '+' => v.push(Add(1)),
                '-' => v.push(Add(-1)),
                '.' => v.push(Output),
                ',' => v.push(Input),
                '[' => {
                    // push sentinel and mark it's index as unmatched
                    v.push(JumpIfZero(0));
                    // push sentinel value to jump stack, fixing is done later.
                    check_jumps.push(());
                }
                ']' => {
                    // pop sentinel value from jump stack to check syntax.
                    let _ = check_jumps.pop().expect("syntax error: unmatched ']'");
                    // push sentinel value, will be fixedx later.
                    v.push(JumpIfNonZero(0));
                }
                _ => (),
            }
        }
        v = repeated_instructions_pass(v);

        v
    }
}

fn repeated_instructions_pass(instrs: Vec<Instruction>) -> Vec<Instruction> {
    use Instruction::*;
    let mut instrs = instrs.into_iter();
    let mut v = Vec::with_capacity(instrs.len());
    match instrs.next() {
        Some(first) => v.push(first),
        None => return v,
    }
    while let Some(i) = instrs.next() {
        match (v.last_mut().unwrap(), i) {
            (MovePointer(n), MovePointer(m)) | (Add(n), Add(m)) => *n += m,
            _ => v.push(i),
        }
    }
    // fix jumps after the pass
    let mut jump_pairs = HashMap::new();
    let mut jumps = vec![];
    for (idx, instr) in v.iter().enumerate() {
        match instr {
            JumpIfZero(_) => jumps.push(idx),
            JumpIfNonZero(_) => {
                let pair = jumps.pop().expect("unmatched [] after passes.");
                jump_pairs.insert(pair, idx);
            }
            _ => (),
        }
    }
    for (p1, p2) in jump_pairs.into_iter() {
        v[p1] = JumpIfZero(p2);
        v[p2] = JumpIfNonZero(p1);
    }
    v
}

type Cell = u8;

pub struct VM {
    instructions: Vec<Instruction>,
    ip: usize,
    cells: HashMap<usize, Cell>,
    ptr: usize,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> VM {
        VM {
            instructions,
            ip: 0,
            cells: HashMap::new(),
            ptr: 0,
        }
    }

    fn fetch(&mut self) -> Option<Instruction> {
        let instr = self.instructions.get(self.ip).cloned();
        self.ip += 1;
        instr
    }

    fn get_cell_mut(&mut self, idx: usize) -> &mut Cell {
        self.cells.entry(idx).or_insert(0)
    }

    fn get_cell(&mut self, idx: usize) -> &Cell {
        self.cells.entry(idx).or_insert(0)
    }

    fn getchar() -> Cell {
        let mut c = [0];
        let mut stdin = io::stdin();
        match stdin.read(&mut c) {
            Err(e) if !matches!(e.kind(), ErrorKind::UnexpectedEof) => panic!("{}", e),
            _ => (),
        }
        c[0]
    }

    fn putchar(c: Cell) {
        let c = char::from(c);
        print!("{}", c)
    }

    pub fn execute(mut self) {
        use Instruction::*;
        while let Some(instr) = self.fetch() {
            match instr {
                MovePointer(n) => {
                    self.ptr = if n > 0 {
                        self.ptr.wrapping_add(n.unsigned_abs())
                    } else {
                        self.ptr.wrapping_sub(n.unsigned_abs())
                    }
                }
                Add(n) => {
                    let cell = self.get_cell_mut(self.ptr);
                    *cell = if n > 0 {
                        cell.wrapping_add(n.try_into().unwrap())
                    } else {
                        cell.wrapping_sub(n.unsigned_abs().try_into().unwrap())
                    }
                }
                Input => {
                    self.cells.insert(self.ptr, VM::getchar());
                }
                Output => VM::putchar(*self.get_cell(self.ptr)),
                JumpIfZero(i) => {
                    if self.get_cell(self.ptr) == &0 {
                        self.ip = i
                    }
                }
                JumpIfNonZero(i) => {
                    if self.get_cell(self.ptr) != &0 {
                        self.ip = i
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_instructions_pass() {
        use Instruction::*;
        assert_eq!(
            repeated_instructions_pass(vec![MovePointer(1), MovePointer(-1), MovePointer(1)]),
            vec![MovePointer(1)]
        );
        assert_eq!(
            repeated_instructions_pass(vec![Add(1), Add(1), Add(-1),]),
            vec![Add(1)]
        );
    }
}
