use mem::object_address_list::ObjectAddressList;
use mem::region::Region;

pub fn mark(mut roots: Vec<u32>, addresses: ObjectAddressList, region: Region) {
    unimplemented!();
}

pub fn sweep() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use mem::object_address_list::ObjectAddressList;
    use mem::region::Region;

    #[test]
    fn mark_traverses_and_marks() {
        let roots: Vec<u32> = Vec::new();
        let addresses: ObjectAddressList = ObjectAddressList::new();
        //let region: Region = Region::new();
    }
}
