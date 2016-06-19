extern crate kwon_vm;

use kwon_vm::*;

fn main() {

    let code = vec![
        // assign a value to t0
        Instruction::new(Op::Assign, 1337, 0, 0),
        // assign a value to t1
        Instruction::new(Op::Assign, 1234, 1, 0),
        // add t0 and t1 and assign it to t2
        Instruction::new(Op::Add, 0, 1, 2),
        // debug t2
        Instruction::new(Op::Debug, 2, 0, 0),
        // terminate
        Instruction::new(Op::Exit, 0, 0, 0)
    ];

    let assembly = Assembly::new(code);

    let mut vm = VirtualMachine::new(assembly);

    vm.run();
}
