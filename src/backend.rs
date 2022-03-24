use crate::Instruction;

pub struct QBE {
    instructions: Vec<Instruction>,
}

impl QBE {
    pub fn new(instructions: Vec<Instruction>) -> QBE {
        QBE { instructions }
    }
    pub fn compile(self) -> String {
        todo!()
    }
}
