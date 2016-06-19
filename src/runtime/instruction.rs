use runtime::op::Op;

#[derive(Debug)]
pub struct Instruction {
    pub op: Op,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32
}

impl Instruction {
    pub fn new(op: Op, arg1: u32, arg2: u32, arg3: u32) -> Instruction {
        return Instruction { op: op, arg1: arg1, arg2: arg2, arg3: arg3 };
    }
}
