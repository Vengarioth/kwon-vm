use runtime::interpreter::RegisterAllocator;

pub struct MemoryRegisterAllocator {
    reg0: u32,
    reg1: u32,
    reg2: u32,
    reg3: u32
}

impl RegisterAllocator for MemoryRegisterAllocator {
    fn set(&mut self, register: u32, value: u32) -> Result<(), &'static str> {
        match(register) {
            0 => {
                self.reg0 = value;
                return Ok(());
            }
            1 => {
                self.reg1 = value;
                return Ok(());
            }
            2 => {
                self.reg2 = value;
                return Ok(());
            }
            3 => {
                self.reg3 = value;
                return Ok(());
            }
            _ => {
                return Err("register out of bounds");
            }
        }
    }

    fn get(&self, register: u32) -> Result<u32, &'static str> {
        match(register) {
            0 => {
                return Ok(self.reg0);
            }
            1 => {
                return Ok(self.reg1);
            }
            2 => {
                return Ok(self.reg2);
            }
            3 => {
                return Ok(self.reg3);
            }
            _ => {
                return Err("register out of bounds");
            }
        }
    }

    fn get_register_count(&self) -> u32 {
        return 4;
    }
}
