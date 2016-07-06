use runtime::interpreter::ObjectAllocator;

pub struct MemoryObjectPool {

}

impl MemoryObjectPool {
    pub fn new() -> MemoryObjectPool {
        return MemoryObjectPool { };
    }
}

impl ObjectAllocator for MemoryObjectPool {

}
