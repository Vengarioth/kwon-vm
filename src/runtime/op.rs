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
    Exit,

    Increment,          //inc x
    Decrement,          //dec x

    Jump,               //jump x
    JumpOnEquality,     //if x == y jump z
    JumpOnInequality,   //if x != y jump z
    JumpIfGreater,      //if x > y jump z
    JumpIfLess,         //if x < y jump z
}
