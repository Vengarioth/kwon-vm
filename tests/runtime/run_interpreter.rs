use kwon_vm::*;

#[test]
fn test_exit() {

    let mut register = MemoryRegisterAllocator::new();
    let mut constant_pool = MemoryConstantPool::new();
    let mut object_pool = MemoryObjectPool::new();
    let mut program = MemoryProgram::new();

    program.push_instruction(Instruction::new(Op::Exit as u8, 0, 0, 0));

    let mut interpreter = Interpreter::new(register, constant_pool, object_pool, program);

    interpreter.fetch();
    interpreter.decode();
    assert_eq!(interpreter.execute(), false);
}

#[test]
fn test_assign() {

    let mut register = MemoryRegisterAllocator::new();
    let mut constant_pool = MemoryConstantPool::new();
    let mut object_pool = MemoryObjectPool::new();
    let mut program = MemoryProgram::new();

    program.push_instruction(Instruction::new(Op::Assign as u8, 10, 0, 0));
    program.push_instruction(Instruction::new(Op::Exit as u8, 0, 0, 0));

    let mut interpreter = Interpreter::new(register, constant_pool, object_pool, program);

    interpreter.fetch();
    interpreter.decode();
    assert_eq!(interpreter.execute(), true);

    interpreter.fetch();
    interpreter.decode();
    assert_eq!(interpreter.execute(), false);
}
