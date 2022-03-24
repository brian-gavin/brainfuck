use std::{
    collections::HashMap,
    io::{self, ErrorKind, Read},
};

use crate::Instruction;

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
