use std::{
    collections::HashMap,
    io::{self, ErrorKind, Read},
};

pub struct Compiler<I>
where
    I: Iterator<Item = char>,
{
    input: I,
    jumps: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    MovePointerLeft,
    MovePointerRight,
    Increment,
    Decrement,
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
        Compiler {
            input,
            jumps: vec![],
        }
    }

    pub fn compile(mut self) -> Vec<Instruction> {
        use Instruction::*;
        let mut v = vec![];
        while let Some(c) = self.input.next() {
            match c {
                '>' => v.push(MovePointerRight),
                '<' => v.push(MovePointerLeft),
                '+' => v.push(Increment),
                '-' => v.push(Decrement),
                '.' => v.push(Output),
                ',' => v.push(Input),
                '[' => {
                    // push sentinel and mark it's index as unmatched
                    v.push(JumpIfZero(0));
                    // jump target is the index of this value
                    self.jumps.push(v.len() - 1);
                }
                ']' => {
                    // pop the last value from the jumps stack to pair the jumps
                    let pair = self.jumps.pop().expect("syntax error: unmatched ']'");
                    v.push(JumpIfNonZero(pair));
                    let jump_target = v.len(); // jump to one-after this instr
                    v[pair] = JumpIfZero(jump_target);
                }
                _ => (),
            }
        }
        v
    }
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
                MovePointerLeft => self.ptr = self.ptr.wrapping_sub(1),
                MovePointerRight => self.ptr = self.ptr.wrapping_add(1),
                Increment => {
                    let cell = self.get_cell_mut(self.ptr);
                    *cell = cell.wrapping_add(1);
                }
                Decrement => {
                    let cell = self.get_cell_mut(self.ptr);
                    *cell = cell.wrapping_sub(1);
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
