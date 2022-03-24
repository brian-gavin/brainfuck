use std::collections::HashMap;

use crate::Instruction;
pub struct Compiler<I>
where
    I: Iterator<Item = char>,
{
    input: I,
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
