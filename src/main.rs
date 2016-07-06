extern crate kwon_vm;

use kwon_vm::*;

/**

Simple test program

for(int i = 0; i < 10; i++) {
  console.log(i);
}

represented as

                Assign t0 = 0;
                Assign t1 = 9;
Label 0;        Nop;
                JumpIfGreater t0, t1, 1;
                Debug t0;
                Increment t0;
                Jump 0;
Label 1;        Nop;
                Exit;

**/

fn main() {
    let mut register = MemoryRegisterAllocator::new();
    let mut constant_pool = MemoryConstantPool::new();
    let mut object_pool = MemoryObjectPool::new();
    let mut program = MemoryProgram::new();

    program.push_instruction(Instruction::new(Op::Assign as u8, 0, 0, 0));
    program.push_instruction(Instruction::new(Op::Assign as u8, 9, 1, 0));
    program.push_instruction(Instruction::new(Op::Nop as u8, 0, 0, 0));
    program.push_instruction(Instruction::new(Op::JumpIfGreater as u8, 0, 1, 7));
    program.push_instruction(Instruction::new(Op::Debug as u8, 0, 0, 0));
    program.push_instruction(Instruction::new(Op::Increment as u8, 0, 0, 0));
    program.push_instruction(Instruction::new(Op::Jump as u8, 2, 0, 0));
    program.push_instruction(Instruction::new(Op::Nop as u8, 0, 0, 0));
    program.push_instruction(Instruction::new(Op::Exit as u8, 0, 0, 0));

    let mut interpreter = Interpreter::new(register, constant_pool, object_pool, program);

    loop {
        interpreter.fetch();
        interpreter.decode();
        if !interpreter.execute() {
            break;
        }
    }
}
