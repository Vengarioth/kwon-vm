use kwon_vm::mem::util::*;

#[test]
fn test_align() {
    assert_eq!(align(0, 16), 0);
    assert_eq!(align(5, 16), 16);
    assert_eq!(align(15, 16), 16);
    assert_eq!(align(16, 16), 16);
    assert_eq!(align(17, 16), 32);
    assert_eq!(align(31, 16), 32);
    assert_eq!(align(32, 16), 32);
    assert_eq!(align(33, 16), 48);
}
