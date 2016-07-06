use runtime::interpreter::ConstantPool;

pub struct MemoryConstantPool {

}

impl MemoryConstantPool {
    pub fn new() -> MemoryConstantPool {
        return MemoryConstantPool { };
    }
}

impl ConstantPool for MemoryConstantPool {

}
