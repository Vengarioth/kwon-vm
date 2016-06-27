use runtime::op::Op;

pub trait ConstantPool {
    fn fetch_string(&self, string_id: u32) -> Result<&[u8], &'static str>;
    fn fetch_constant(&self, constant_id: u32) -> Result<&[u8], &'static str>;
    fn fetch_function(&self, function_id: u32) -> Result<&[u8], &'static str>;
}

pub trait RegisterAllocator {
    fn set(&mut self, register: u32, value: u32) -> Result<(), &'static str>;
    fn get(&self, register: u32) -> Result<u32, &'static str>;
    fn get_register_count(&self) -> u32;
}

pub trait ObjectAllocator {
    fn allocate_structure(structure_type: u32) -> Result<u32, &'static str>;
}

pub struct Interpreter<R: RegisterAllocator, C: ConstantPool, O: ObjectAllocator> {
    program_pointer: u32,
    operation: Op,
    address1: u32,
    address2: u32,
    address3: u32,
    scalar: u32,
    constants: C,
    register: R,
    objects: O
}

impl<R: RegisterAllocator, C: ConstantPool, O: ObjectAllocator> Interpreter<R, C, O> {

    pub fn new(register_allocator: R, constant_pool: C, object_allocator: O) -> Interpreter<R, C, O> {
        return Interpreter {
            program_pointer: 0,
            operation: Op::Nop,
            address1: 0,
            address2: 0,
            address3: 0,
            scalar: 0,
            constants: constant_pool,
            register: register_allocator,
            objects: object_allocator
        };
    }

    #[inline(always)]
    pub fn fetch(&mut self) {

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
                println!("Debug: {}", self.register.get(self.address1).unwrap());
            }
            Op::Exit => {
                continue_execution = false;
            }
        }

        self.program_pointer += 1;
        return continue_execution;
    }
}
