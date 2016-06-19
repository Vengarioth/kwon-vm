#[repr(u8)]
#[allow(dead_code)]
#[allow(unused_qualifications)]
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Nop,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Debug,
    Exit
}
