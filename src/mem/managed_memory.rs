use mem::util;
use mem::object_address_list::ObjectAddressList;

pub trait Allocator {
    fn malloc(&self, size: usize) -> Result<*mut u8, &'static str>;
    fn free(&self, ptr: *mut u8) -> Result<(), &'static str>;
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
    address_list: ObjectAddressList,
    allocator: A,
}

impl<A: Allocator> ManagedMemory<A> {

    pub fn new(region_size: u32, cell_size: u32, allocator: A) -> ManagedMemory<A> {
        return ManagedMemory {
            region_size: region_size,
            cell_size: cell_size,
            regions: Vec::new(),
            address_list: ObjectAddressList::new(),
            allocator: allocator
        };
    }

    pub fn allocate(&mut self, size: u32) -> Result<u32, &'static str> {
        if(size < 1) {
            return Err("Size must be bigger than zero.");
        }
        let aligned_size = util::align(size, self.cell_size);


        //TODO place in region memory
        let dummy: u8 = 0;
        let ptr = &dummy as *const u8;


        let address = self.address_list.store_memory_address(ptr).unwrap();
        return Ok(address);
    }

    fn allocate_region(&self) -> Result<Region, &'static str> {
        return Ok(Region::new(0, 0));
    }

}
