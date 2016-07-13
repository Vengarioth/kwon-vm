use std::ptr;

/// # ObjectAddressList
/// Stores memory addresses of objects so that objects can be relocated in memory while keeping a single unique address
pub struct ObjectAddressList {
    addresses: Vec<*const u8>,
    unused: Vec<u32>
}

impl ObjectAddressList {

    /// Creates a new instance of ObjectAddressList
    pub fn new() -> ObjectAddressList {
        return ObjectAddressList {
            addresses: Vec::new(),
            unused: Vec::new()
        };
    }

    /// returns the address in memory for the given object
    pub fn get_memory_address(&self, object_address: u32) -> Option<*const u8> {
        if object_address as usize >= self.addresses.len() ||
            self.addresses[object_address as usize].is_null() {
            return None;
        }

        return Some(self.addresses[object_address as usize]);
    }

    /// stores an object and returns a unique identifier
    pub fn store_memory_address(&mut self, memory_address: *const u8) -> Result<u32, &'static str> {
        let object_address: u32;
        if self.unused.len() > 0 {
            object_address = self.unused.pop().unwrap();
            self.addresses[object_address as usize] = memory_address;
        }else{
            self.addresses.push(memory_address);
            object_address = (self.addresses.len() as u32) - 1;
        }

        return Ok(object_address);
    }

    /// updates the memory address of a stored object
    pub fn update_memory_address(&mut self, object_address: u32, new_memory_address: *const u8) -> Result<(), &'static str> {
        if object_address as usize >= self.addresses.len() ||
            self.addresses[object_address as usize].is_null() {
            return Err("object not found");
        }

        self.addresses[object_address as usize] = new_memory_address;
        return Ok(());
    }

    /// clears a stored object from this list
    pub fn clear_object(&mut self, object_address: u32) -> Result<(), &'static str> {
        if object_address as usize >= self.addresses.len() ||
            self.addresses[object_address as usize].is_null() {
            return Err("object not found");
        }

        self.addresses[object_address as usize] = ptr::null();
        self.unused.push(object_address);

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stores_memory_addresses() {
        let mut address_list = ObjectAddressList::new();
        let x: u8 = 5;
        let mem_address = &x as *const u8;

        let address = address_list.store_memory_address(mem_address).unwrap();

        assert_eq!(mem_address, address_list.get_memory_address(address).unwrap());
        assert_eq!(5, unsafe { *address_list.get_memory_address(address).unwrap() });
    }

    #[test]
    fn it_returns_none_for_non_existant_objects() {
        let mut address_list = ObjectAddressList::new();
        assert_eq!(None, address_list.get_memory_address(123));
    }

    #[test]
    fn it_updates_memory_addresses() {
        let mut address_list = ObjectAddressList::new();
        let x: u8 = 5;
        let mem_address = &x as *const u8;

        let y: u8 = 10;
        let mem_address_two = &y as *const u8;

        let address = address_list.store_memory_address(mem_address).unwrap();
        address_list.update_memory_address(address, mem_address_two);

        assert_eq!(mem_address_two, address_list.get_memory_address(address).unwrap());
        assert_eq!(10, unsafe { *address_list.get_memory_address(address).unwrap() });
    }

    #[test]
    fn it_clears_stored_objects() {
        let mut address_list = ObjectAddressList::new();
        let x: u8 = 5;
        let mem_address = &x as *const u8;

        let address = address_list.store_memory_address(mem_address).unwrap();

        address_list.clear_object(address);

        assert_eq!(None, address_list.get_memory_address(address));
    }

    #[test]
    fn it_stores_multiple_objects() {
        let mut address_list = ObjectAddressList::new();
        let val_one: u8 = 5;
        let val_two: u8 = 3;
        let val_three: u8 = 7;
        let val_four: u8 = 9;
        let mem_address_one = &val_one as *const u8;
        let mem_address_two = &val_two as *const u8;
        let mem_address_three = &val_three as *const u8;
        let mem_address_four = &val_four as *const u8;

        let address_one = address_list.store_memory_address(mem_address_one).unwrap();
        let address_two = address_list.store_memory_address(mem_address_two).unwrap();
        let address_three = address_list.store_memory_address(mem_address_three).unwrap();
        let address_four = address_list.store_memory_address(mem_address_four).unwrap();

        assert_eq!(5, unsafe { *address_list.get_memory_address(address_one).unwrap() });
        assert_eq!(3, unsafe { *address_list.get_memory_address(address_two).unwrap() });
        assert_eq!(7, unsafe { *address_list.get_memory_address(address_three).unwrap() });
        assert_eq!(9, unsafe { *address_list.get_memory_address(address_four).unwrap() });
    }
}
