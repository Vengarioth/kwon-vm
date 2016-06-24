pub mod data;
mod runtime;
mod assembly;
mod virtual_machine;
mod io;

pub use data::assembly::Assembly;
pub use virtual_machine::VirtualMachine;
pub use runtime::op::Op;
pub use runtime::instruction::Instruction;
