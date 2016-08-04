use std::ptr;
use libc;
use util::bit_util;

const CELL_SIZE_IN_BYTES: u32 = 16;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    BlockExtend,
    Free,
    WhiteBlock,
    BlackBlock,
    MetaData
}

pub struct Region {
    ptr: *const u8,
    block_ptr: *mut u8,
    mark_ptr: *mut u8,
    meta_offset: u32,
    bump_offset: u32,
    cell_count: u32,
    size_in_bytes: u32
}

impl Region {
    pub fn new(cell_count: u32) -> Result<Region, &'static str> {
        if cell_count < 64 {
            return Err("cell_count must at least be 64");
        }
        if (cell_count & (cell_count - 1)) != 0 {
            return Err("cell_count must be a power of two");
        }

        let size_in_bytes = cell_count * CELL_SIZE_IN_BYTES;
        let required_meta_space: u32 = (cell_count as f32 / (CELL_SIZE_IN_BYTES as f32 * 4.0)).ceil() as u32;
        let meta_offset = required_meta_space * 2;

        let ptr: *mut u8 = unsafe { libc::malloc(size_in_bytes as libc::size_t) as *mut u8 };
        if ptr.is_null() {
            return Err("Could not allocate memory for region");
        }

        let block_ptr: *mut u8 = unsafe { ptr.offset(0) };
        let mark_ptr: *mut u8 = unsafe { ptr.offset((CELL_SIZE_IN_BYTES * required_meta_space) as isize) };

        for i in 0..cell_count {
            bit_util::unset(i as usize, block_ptr);
            bit_util::set(i as usize, mark_ptr);
        }

        return Ok(Region {
            ptr: ptr,
            cell_count: cell_count,
            size_in_bytes: size_in_bytes,
            meta_offset: meta_offset,
            bump_offset: meta_offset,
            block_ptr: block_ptr,
            mark_ptr: mark_ptr
        });
    }

    pub fn free(&mut self) -> Result<(), &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        unsafe { libc::free(self.ptr as *mut libc::c_void) };
        self.ptr = ptr::null();
        return Ok(());
    }

    pub fn get_memory_address(&self, address: u32) -> Result<*const u8, &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        if address >= self.cell_count {
            return Err("address is out of bounds");
        }

        return Ok(unsafe { self.ptr.offset((CELL_SIZE_IN_BYTES * address) as isize) });
    }

    pub fn has_free_cells(&self, size: u32) -> Result<bool, &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        return Ok(self.has_free_cells_bump(size));
    }

    pub fn claim(&mut self, size: u32) -> Result<u32, &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        return self.claim_bump(size);
    }

    pub fn release(&mut self, address: u32) -> Result<(), &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        if address >= self.cell_count {
            return Err("address is out of bounds");
        }

        if !bit_util::is_set(address as usize, self.block_ptr) {
            return Err("no block at address");
        }

        let start = address + 1;

        bit_util::unset(address as usize, self.block_ptr);
        bit_util::set(address as usize, self.mark_ptr);

        for i in start..self.cell_count {
            if bit_util::is_set(i as usize, self.block_ptr) || bit_util::is_set(i as usize, self.mark_ptr) {
                break;
            }

            bit_util::unset(i as usize, self.block_ptr);
            bit_util::set(i as usize, self.mark_ptr);
        }

        return Ok(());
    }

    fn has_free_cells_bump(&self, size: u32) -> bool {
        return self.cell_count - self.bump_offset < size;
    }

    fn claim_bump(&mut self, size: u32) -> Result<u32, &'static str> {
        if self.cell_count - self.bump_offset < size {
            return Err("Not enough space in region");
        }

        let address = self.bump_offset;
        self.bump_offset += size;

        bit_util::set(address as usize, self.block_ptr);
        for i in address..self.bump_offset {
            bit_util::unset(i as usize, self.mark_ptr);
        }

        return Ok(address);
    }

    pub fn get_cell_state(&self, address: u32) -> Result<CellState, &'static str> {
        if self.ptr.is_null() {
            return Err("region is already freed");
        }

        if address >= self.cell_count {
            return Err("address is out of bounds");
        }

        if address < self.meta_offset {
            return Ok(CellState::MetaData);
        }

        let block: bool = bit_util::is_set(address as usize, self.block_ptr);
        let mark: bool = bit_util::is_set(address as usize, self.mark_ptr);

        if !block && !mark {
            return Ok(CellState::BlockExtend);
        }

        if !block && mark {
            return Ok(CellState::Free);
        }

        if block && !mark {
            return Ok(CellState::WhiteBlock);
        }

        return Ok(CellState::BlackBlock);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_region() {
        let mut region = Region::new(64).unwrap();

        assert_eq!(region.get_cell_state(0).unwrap(), CellState::MetaData);
        assert_eq!(region.get_cell_state(1).unwrap(), CellState::MetaData);

        for i in 2..64 {
            assert_eq!(region.get_cell_state(i).unwrap(), CellState::Free);
        }

        region.free().unwrap();
    }

    #[test]
    fn it_can_claim_and_release_cells() {
        let mut region = Region::new(64).unwrap();

        let address = region.claim(10).unwrap();

        assert_eq!(region.get_cell_state(0).unwrap(), CellState::MetaData);
        assert_eq!(region.get_cell_state(1).unwrap(), CellState::MetaData);

        assert_eq!(region.get_cell_state(2).unwrap(), CellState::WhiteBlock);

        for i in 3..11 {
            assert_eq!(region.get_cell_state(i).unwrap(), CellState::BlockExtend);
        }

        assert_eq!(region.get_cell_state(12).unwrap(), CellState::Free);

        region.release(address);

        for i in 2..64 {
            assert_eq!(region.get_cell_state(i).unwrap(), CellState::Free);
        }

        region.free().unwrap();
    }

}
