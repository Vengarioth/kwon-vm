use kwon_vm::data::instruction::*;

#[test]
fn create_instruction() {
    let instruction = Instruction::new(0, 1, 2, 3);

    assert_eq!(instruction.op, 0);
    assert_eq!(instruction.arg1, 1);
    assert_eq!(instruction.arg2, 2);
    assert_eq!(instruction.target, 3);
}

#[test]
fn create_scalar_instruction() {
    let instruction = Instruction::new_scalar(0, 1234, 1);

    assert_eq!(instruction.op, 0);
    assert_eq!(instruction.target, 1);
    assert_eq!(instruction.get_scalar(), 1234);
}

#[test]
fn create_from_binary() {
    let binary: [u8; 4] = [4, 5, 6, 7];

    let instruction = Instruction::from_binary(&binary[0..4]);

    assert_eq!(instruction.op, 4);
    assert_eq!(instruction.arg1, 5);
    assert_eq!(instruction.arg2, 6);
    assert_eq!(instruction.target, 7);
}
