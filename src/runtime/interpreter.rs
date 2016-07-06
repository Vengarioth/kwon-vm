use std::mem::transmute;
use runtime::op::Op;

pub trait ConstantPool {
    //fn fetch_string(&self, string_id: u32) -> Result<&[u8], &'static str>;
    //fn fetch_constant(&self, constant_id: u32) -> Result<&[u8], &'static str>;
    //fn fetch_function(&self, function_id: u32) -> Result<&[u8], &'static str>;
}

pub trait RegisterAllocator {
    fn set(&mut self, register: u32, value: u32) -> Result<(), &'static str>;
    fn get(&self, register: u32) -> Result<u32, &'static str>;
    fn get_register_count(&self) -> u32;
}

pub trait ObjectAllocator {

}

pub trait Program {
    fn read(&self, position: u32) -> Result<[u8; 4], &'static str>;
}

pub struct Interpreter<R: RegisterAllocator, C: ConstantPool, O: ObjectAllocator, P:Program> {
    program_pointer: u32,
    instruction: [u8; 4],
    operation: Op,
    address1: u32,
    address2: u32,
    address3: u32,
    scalar: u32,
    constants: C,
    register: R,
    objects: O,
    program: P
}

impl<R: RegisterAllocator, C: ConstantPool, O: ObjectAllocator, P: Program> Interpreter<R, C, O, P> {

    pub fn new(register_allocator: R, constant_pool: C, object_allocator: O, program: P) -> Interpreter<R, C, O, P> {
        return Interpreter {
            program_pointer: 0,
            instruction: [0, 0, 0, 0],
            operation: Op::Nop,
            address1: 0,
            address2: 0,
            address3: 0,
            scalar: 0,
            constants: constant_pool,
            register: register_allocator,
            objects: object_allocator,
            program: program
        };
    }

    #[inline(always)]
    pub fn fetch(&mut self) {
        self.instruction = self.program.read(self.program_pointer).unwrap();
    }

    #[inline(always)]
    pub fn decode(&mut self) {
        self.operation = unsafe { transmute(self.instruction[0]) };
        self.address1 = self.instruction[1] as u32;
        self.address2 = self.instruction[2] as u32;
        self.address3 = self.instruction[3] as u32;

        let scalar_data: [u8; 2] = [self.instruction[1], self.instruction[2]];
        let scalar: u16 = unsafe { transmute(scalar_data) };
        self.scalar = scalar as u32;
    }

    #[inline(always)]
    pub fn execute(&mut self) -> bool {
        let mut continue_execution = true;
        match self.operation {
            Op::Nop => {}
            Op::Assign => {
                self.register.set(self.address2, self.address1).unwrap();
            }
            Op::Add => {
                let t1 = self.register.get(self.address1).unwrap();
                let t2 = self.register.get(self.address2).unwrap();
                self.register.set(self.address3, t2 + t1).unwrap();
            }
            Op::Sub => {
                let t1 = self.register.get(self.address1).unwrap();
                let t2 = self.register.get(self.address2).unwrap();
                self.register.set(self.address3, t2 - t1).unwrap();
            }
            Op::Mul => {
                let t1 = self.register.get(self.address1).unwrap();
                let t2 = self.register.get(self.address2).unwrap();
                self.register.set(self.address3, t2 * t1).unwrap();
            }
            Op::Div => {
                let t1 = self.register.get(self.address1).unwrap();
                let t2 = self.register.get(self.address2).unwrap();
                self.register.set(self.address3, t2 / t1).unwrap();
            }
            Op::Debug => {
                println!("{}", self.register.get(self.address1).unwrap());
            }
            Op::Exit => {
                continue_execution = false;
            }
            Op::Increment => {
                let t1 = self.register.get(self.address1).unwrap();
                self.register.set(self.address1, t1 + 1).unwrap();
            }
            Op::Jump => {
                self.program_pointer = self.address1;
            }
            Op::JumpIfGreater => {
                let t1 = self.register.get(self.address1).unwrap();
                let t2 = self.register.get(self.address2).unwrap();
                if t1 > t2 {
                    self.program_pointer = self.address3;
                }
            }
            _ => {}
        }

        self.program_pointer += 1;
        return continue_execution;
    }
}
