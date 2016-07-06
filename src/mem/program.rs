use runtime::interpreter::Program;
use data::instruction::Instruction;

pub struct MemoryProgram {
    data: Vec<u8>
}

impl MemoryProgram {
    pub fn new() -> MemoryProgram {
        let mut data: Vec<u8> = Vec::new();
        return MemoryProgram {
            data: data
        };
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        let binary_data = instruction.get_binary();
        self.data.push(binary_data[0]);
        self.data.push(binary_data[1]);
        self.data.push(binary_data[2]);
        self.data.push(binary_data[3]);
    }
}

impl Program for MemoryProgram {
    fn read(&self, position: u32) -> Result<[u8; 4], &'static str> {
        let aligned_position = position * 4;

        if aligned_position + 4 > self.data.len() as u32 {
            return Err("position out of bounds");
        }

        let data: [u8; 4] = [
            self.data[aligned_position as usize],
            self.data[(aligned_position + 1) as usize],
            self.data[(aligned_position + 2) as usize],
            self.data[(aligned_position + 3) as usize]];
        return Ok(data);
    }
}
