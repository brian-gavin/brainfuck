use crate::{Instruction, NUM_CELLS};
use std::fmt::{self, Write};

pub struct QBE {
    instructions: Vec<Instruction>,
    text: String,
}

impl QBE {
    pub fn new(instructions: Vec<Instruction>) -> QBE {
        QBE {
            instructions,
            text: String::from(BUILTINS),
        }
    }

    pub fn compile(mut self) -> Result<String, fmt::Error> {
        writeln!(self.text, "data $cells = {{ z {} }}", NUM_CELLS)?;
        writeln!(self.text, "data $pointer = {{ l 1 }}")?;
        writeln!(self.text, "")?;
        writeln!(self.text, "export function w $main() {{")?;
        writeln!(self.text, "@start")?;
        self.compile_main()?;
        writeln!(self.text, "    ret 0")?;
        writeln!(self.text, "}}")?;
        Ok(self.text)
    }

    fn compile_main(&mut self) -> Result<(), fmt::Error> {
        let text = &mut self.text;
        for (i, instr) in self.instructions.iter().enumerate() {
            match instr {
                Instruction::MovePointer(n) => {
                    writeln!(text, "    call $move_pointer(l {})", n)?;
                }
                Instruction::Add(n) => {
                    writeln!(text, "    call $add(w {})", n)?;
                }
                Instruction::Input => {
                    writeln!(text, "    call $input()")?;
                }
                Instruction::Output => {
                    writeln!(text, "    call $output()")?;
                }
                Instruction::JumpIfZero(target) => {
                    writeln!(text, "@L{}", i)?;
                    writeln!(text, "    %p =l call $get_cell_addr()")?;
                    writeln!(text, "    %c =w loadsb %p")?;
                    writeln!(text, "    %t =w ceqw %c, 0")?;
                    writeln!(text, "    jnz %t, @L{}, @L{}.fallthrough", target, i)?;
                    writeln!(text, "@L{}.fallthrough", i)?;
                }
                Instruction::JumpIfNonZero(target) => {
                    writeln!(text, "    %p =l call $get_cell_addr()")?;
                    writeln!(text, "    %c =w loadsb %p")?;
                    writeln!(text, "    jnz %c, @L{}, @L{}", target, i)?;
                    writeln!(text, "@L{}", i)?;
                }
            }
        }
        Ok(())
    }
}

const BUILTINS: &'static str = r#"
function l $get_cell_addr() {
@start
    %p =l loadl $pointer
    %cellp =l add $cells, %p
    ret %cellp
}

function $move_pointer(l %n) {
@start
    %p =l loadl $pointer
    %p =l add %p, %n
    storel %p, $pointer
    ret
}

function $add(w %n) {
@start
    %p =l call $get_cell_addr()
    %c =w loadsb %p
    %c =w add %c, %n
    storeb %c, %p
    ret
}

function $input() {
@start
    %c =w call $getchar()
    %t =w ceqw %c, -1
    jnz %t, @is_eof, @is_not_eof
@is_eof
    %c =w xor %c, %c
@is_not_eof
    %p =l call $get_cell_addr()
    storew %c, %p
    ret
}

function $output() {
@start
    %p =l call $get_cell_addr()
    %c =w loadsb %p
    call $putchar(w %c)
    ret
}
"#;
