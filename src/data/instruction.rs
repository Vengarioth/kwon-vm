use std::mem::transmute;

#[derive(Debug)]
pub struct Instruction {
    pub op: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub target: u8
}

impl Instruction {
    pub fn new(op: u8, arg1: u8, arg2: u8, target: u8) -> Instruction {
        return Instruction { op: op, arg1: arg1, arg2: arg2, target: target};
    }

    pub fn new_scalar(op: u8, scalar: u16, target: u8) -> Instruction {
        let scalar_data: [u8; 2] = unsafe { transmute(u16::to_le(scalar)) };
        return Instruction { op: op, arg1: scalar_data[0], arg2: scalar_data[1], target: target};
    }

    pub fn from_binary(binary: &[u8]) -> Instruction {
        return Instruction { op: binary[0], arg1: binary[1], arg2: binary[2], target: binary[3] };
    }

    pub fn get_scalar(&self) -> u16 {
        let scalar_data: [u8; 2] = [self.arg1, self.arg2];
        let scalar: u16 = u16::from_le(unsafe { transmute(scalar_data) });
        return scalar;
    }
}
