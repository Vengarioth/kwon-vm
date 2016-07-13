use mem::util;

pub trait Allocator {
    fn malloc(&self, size: usize) -> Result<*mut u8, &'static str>;
    fn free(&self, ptr: *mut u8) -> Result<(), &'static str>;
}

pub struct MemoryBlock {
    pub address: u8,
    pub size: u32
}

impl MemoryBlock {
    pub fn new(address: u8, size: u32) -> MemoryBlock {
        return MemoryBlock { address: address, size: size };
    }
}

struct Region {
    address: u32,
    size: u32,
}

impl Region {
    pub fn new(address: u32, size: u32) -> Region {
        return Region { address: address, size: size };
    }
}

pub struct ManagedMemory<A: Allocator> {
    region_size: u32,
    cell_size: u32,
    regions: Vec<Region>,
    allocator: A,
}

impl<A: Allocator> ManagedMemory<A> {

    pub fn new(region_size: u32, cell_size: u32, allocator: A) -> ManagedMemory<A> {
        let regions = Vec::new();
        return ManagedMemory {
            region_size: region_size,
            cell_size: cell_size,
            regions: regions,
            allocator: allocator
        };
    }

    pub fn allocate(&self, size: u32) -> Result<u32, &'static str> {
        if size < 1 {
            return Err("size must be greater than zero");
        }

        let aligned_size = util::align(size, self.cell_size);

        return Ok(0);
    }

    fn allocate_region(&self) -> Result<Region, &'static str> {
        return Ok(Region::new(0, 0));
    }

}
