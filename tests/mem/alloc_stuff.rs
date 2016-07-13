use kwon_vm::mem::{managed_memory, libc_allocator};
use kwon_vm::mem::managed_memory::*;

#[test]
fn alloc_array() {

    println!("");
    println!("----");

    let allocator = libc_allocator::LibcAllocator::new();

    let ptr = allocator.malloc(4).unwrap();
    println!("{:p}", ptr);
    println!("{:p}", unsafe { ptr.offset(1) });

    let mut y = unsafe { *ptr };
    println!("{}", y);

    let mut z = unsafe { *ptr };
    println!("{}", z);

    println!("----");

    //assert_eq!(true, false);
}
