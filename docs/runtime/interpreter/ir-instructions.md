# Intermediate Representation Instructions

This are the instructions available in the __intermediate representation (IR)__. The IR follows rules for __three address code (TAC)__.

## Table of contents
* [Data Structure](#data-structure)
* [Instructions](#instructions)
  * [Assignment](#assignment)
    * [Assign](#assign)
  * [Arithmetic](#arithmetic)
      * [Add](#add)
      * [Sub](#sub)
      * [Mul](#mul)
      * [Div](#div)

## Data Structure

Our _TAC_ will be represented by Quadruples (meaning 4 values make up one instruction). The actual memory layout is not final.

```rust
enum Op {
  //...
}

struct Instruction {
  op: Op,
  arg1: u8,
  arg2: u8,
  arg3: u8
}

// create a new instruction:
Instruction::new(Op::Add, 0, 1, 2);
```

## Instructions

### Assignment
#### Assign
##### Code representation:
```
t2 = t1
```

### Arithmetic

#### Add
##### Code representation:
```
t3 = t2 + t1
```

#### Sub
##### Code representation:
```
t3 = t2 - t1
```

#### Mul
##### Code representation:
```
t3 = t2 * t1
```

#### Div
##### Code representation:
```
t3 = t2 / t1
```
