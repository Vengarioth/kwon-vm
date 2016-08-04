pub fn set(offset: usize, base_ptr: *mut u8) {
    let bit_offset = offset % 4;
    let byte_offset = (offset - bit_offset) / 4;

    let ptr: *mut u8 = unsafe { base_ptr.offset(byte_offset as isize) };
    let mut byte = unsafe { *ptr };

    byte = byte | 1 << bit_offset;

    unsafe { *ptr = byte };
}

pub fn unset(offset: usize, base_ptr: *mut u8) {
    let bit_offset = offset % 4;
    let byte_offset = (offset - bit_offset) / 4;

    let ptr: *mut u8 = unsafe { base_ptr.offset(byte_offset as isize) };
    let mut byte = unsafe { *ptr };

    byte = byte & !(1 << bit_offset);

    unsafe { *ptr = byte };
}

pub fn is_set(offset: usize, base_ptr: *const u8) -> bool {
    let bit_offset = offset % 4;
    let byte_offset = (offset - bit_offset) / 4;

    let byte = unsafe { *base_ptr.offset(byte_offset as isize) };

    return (byte & (1 << bit_offset)) > 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc;

    #[test]
    fn it_sets_bits() {
        let ptr: *mut u8 = unsafe { libc::malloc(64 as libc::size_t) as *mut u8 };

        assert_eq!(is_set(0, ptr), false);
        set(0, ptr);
        assert_eq!(is_set(0, ptr), true);
        unset(0, ptr);
        assert_eq!(is_set(0, ptr), false);

        assert_eq!(is_set(2, ptr), false);
        set(2, ptr);
        assert_eq!(is_set(2, ptr), true);
        unset(2, ptr);
        assert_eq!(is_set(2, ptr), false);

        assert_eq!(is_set(5, ptr), false);
        set(5, ptr);
        assert_eq!(is_set(5, ptr), true);
        unset(5, ptr);
        assert_eq!(is_set(5, ptr), false);

        assert_eq!(is_set(33, ptr), false);
        set(33, ptr);
        assert_eq!(is_set(32, ptr), false);
        assert_eq!(is_set(34, ptr), false);
        assert_eq!(is_set(33, ptr), true);
        unset(33, ptr);
        assert_eq!(is_set(32, ptr), false);
        assert_eq!(is_set(34, ptr), false);
        assert_eq!(is_set(33, ptr), false);

        unsafe { libc::free(ptr as *mut libc::c_void) };
    }

}
