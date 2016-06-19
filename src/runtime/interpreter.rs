use assembly::Assembly;
use runtime::op::Op;

pub struct Interpreter {
    program_pointer: u32,
    operation: Op,
    address1: u32,
    address2: u32,
    address3: u32,
    register: [u32; 16]
}

impl Interpreter {

    pub fn new() -> Interpreter {
        return Interpreter {
            program_pointer: 0,
            operation: Op::Nop,
            address1: 0,
            address2: 0,
            address3: 0,
            register: [0; 16]
        };
    }

    #[inline(always)]
    pub fn fetch(&mut self, assembly: &Assembly) {
        let x = assembly.get_instruction(self.program_pointer);
        self.operation = x.op;
        self.address1 = x.arg1;
        self.address2 = x.arg2;
        self.address3 = x.arg3;
    }

    #[inline(always)]
    pub fn decode(&self) {

    }

    #[inline(always)]
    pub fn execute(&mut self) -> bool {
        let mut continue_execution = true;
        match self.operation {
            Op::Nop => {}
            Op::Assign => {
                self.register[self.address2 as usize] = self.address1;
            }
            Op::Add => {
                let t1 = self.register[self.address1 as usize];
                let t2 = self.register[self.address2 as usize];
                self.register[self.address3 as usize] = t2 + t1;
            }
            Op::Sub => {
                let t1 = self.register[self.address1 as usize];
                let t2 = self.register[self.address2 as usize];
                self.register[self.address3 as usize] = t2 - t1;
            }
            Op::Mul => {
                let t1 = self.register[self.address1 as usize];
                let t2 = self.register[self.address2 as usize];
                self.register[self.address3 as usize] = t2 * t1;
            }
            Op::Div => {
                let t1 = self.register[self.address1 as usize];
                let t2 = self.register[self.address2 as usize];
                self.register[self.address3 as usize] = t2 / t1;
            }
            Op::Debug => {
                println!("Debug: {}", self.register[self.address1 as usize]);
            }
            Op::Exit => {
                continue_execution = false;
            }
        }

        self.program_pointer += 1;
        return continue_execution;
    }
}
