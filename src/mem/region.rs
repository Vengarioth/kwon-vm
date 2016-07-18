use std::ptr;

struct BlockInformation {

}

pub struct Region {
    address: *const u8,
    block_count: u32,
    size_in_bytes: u32,
    block_size_in_bytes: u32
}

impl Region {
    pub fn new(address: *const u8, size_in_bytes: u32, block_size_in_bytes: u32) -> Region {
        let block_count: u32 = size_in_bytes / block_size_in_bytes;

        return Region {
            address: address,
            block_count: block_count,
            size_in_bytes: size_in_bytes,
            block_size_in_bytes: block_size_in_bytes
        };
    }

    pub fn get_memory_address(&self, block: u32) -> Result<*const u8, &'static str> {
        if block >= self.block_count {
            return Err("block is out of bounds");
        }

        return Ok(unsafe { self.address.offset((self.block_size_in_bytes * block) as isize) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
