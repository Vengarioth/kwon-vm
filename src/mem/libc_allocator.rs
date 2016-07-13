use mem::managed_memory::*;
use libc;

pub struct LibcAllocator {
}

impl LibcAllocator {
    pub fn new() -> LibcAllocator {
        return LibcAllocator{};
    }
}

impl Allocator for LibcAllocator {
    fn malloc(&self, size: usize) -> Result<*mut u8, &'static str> {
        let ptr: *mut u8 = unsafe { libc::malloc(size as libc::size_t) as *mut u8 };
        if ptr.is_null() {
            return Err("malloc failed");
        }
        
        return Ok(ptr);
    }

    fn free(&self, ptr: *mut u8) -> Result<(), &'static str> {
        unsafe { libc::free(ptr as *mut libc::c_void) };
        return Ok(());
    }
}
