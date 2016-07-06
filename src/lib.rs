pub mod data;
pub mod runtime;
pub mod io;
pub mod mem;

pub use data::instruction::Instruction;
pub use mem::register::MemoryRegisterAllocator;
pub use mem::constant_pool::MemoryConstantPool;
pub use mem::object_pool::MemoryObjectPool;
pub use mem::program::MemoryProgram;
pub use runtime::interpreter::Interpreter;
pub use runtime::op::Op;
