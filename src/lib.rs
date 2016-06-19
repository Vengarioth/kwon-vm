mod runtime;
mod assembly;
mod virtual_machine;

pub use assembly::Assembly;
pub use virtual_machine::VirtualMachine;
pub use runtime::op::Op;
pub use runtime::instruction::Instruction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
