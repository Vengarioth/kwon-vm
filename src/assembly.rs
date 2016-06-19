use runtime::instruction::Instruction;
use runtime::op::Op;

pub struct Assembly {
    program: Vec<Instruction>
}

impl Assembly {
    pub fn new(program: Vec<Instruction>) -> Assembly {

        return Assembly{ program: program };
    }

    pub fn get_instruction(&self, index: u32) -> &Instruction {
        return &self.program[index as usize];
    }
}
