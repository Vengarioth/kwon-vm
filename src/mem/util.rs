pub fn align(size: u32, alignment: u32) -> u32 {
    if size % alignment == 0 {
        return size;
    }

    return size + (alignment - (size % alignment));
}
