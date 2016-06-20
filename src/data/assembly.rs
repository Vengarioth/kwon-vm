use std::fs::{File, OpenOptions};
use std::io::{Read,BufReader,Write,BufWriter};
use std::mem::transmute;

struct IdToAddressRow {
    id: u32,
    address: u32
}

pub struct Assembly {
    string_ids: Vec<IdToAddressRow>,
    integer_ids: Vec<IdToAddressRow>,
    floating_point_ids: Vec<IdToAddressRow>,
    function_ids: Vec<IdToAddressRow>,
    data: Vec<u8>
}

impl Assembly {

    pub fn new() -> Box<Assembly> {
        let mut string_ids : Vec<IdToAddressRow> = Vec::new();
        let mut integer_ids : Vec<IdToAddressRow> = Vec::new();
        let mut floating_point_ids : Vec<IdToAddressRow> = Vec::new();
        let mut function_ids : Vec<IdToAddressRow> = Vec::new();
        let mut data : Vec<u8> = Vec::new();

        return Box::new(Assembly {
            string_ids: string_ids,
            integer_ids: integer_ids,
            floating_point_ids: floating_point_ids,
            function_ids: function_ids,
            data: data
        });
    }

    pub fn load_from(path: &str) -> Box<Assembly> {

        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        let mut string_ids : Vec<IdToAddressRow> = Vec::new();
        let mut integer_ids : Vec<IdToAddressRow> = Vec::new();
        let mut floating_point_ids : Vec<IdToAddressRow> = Vec::new();
        let mut function_ids : Vec<IdToAddressRow> = Vec::new();
        let mut data : Vec<u8> = Vec::new();

        let string_ids_count = read_u32(&mut reader);
        let integer_ids_count = read_u32(&mut reader);
        let floating_point_ids_count = read_u32(&mut reader);
        let function_ids_count = read_u32(&mut reader);
        let data_length = read_u32(&mut reader);

        for i in 0..string_ids_count {
            let id = read_u32(&mut reader);
            let address = read_u32(&mut reader);
            string_ids.push(IdToAddressRow{id: id, address: address});
        }

        for i in 0..integer_ids_count {
            let id = read_u32(&mut reader);
            let address = read_u32(&mut reader);
            integer_ids.push(IdToAddressRow{id: id, address: address});
        }

        for i in 0..floating_point_ids_count {
            let id = read_u32(&mut reader);
            let address = read_u32(&mut reader);
            floating_point_ids.push(IdToAddressRow{id: id, address: address});
        }

        for i in 0..function_ids_count {
            let id = read_u32(&mut reader);
            let address = read_u32(&mut reader);
            function_ids.push(IdToAddressRow{id: id, address: address});
        }

        reader.read_to_end(&mut data);

        return Box::new(Assembly {
            string_ids: string_ids,
            integer_ids: integer_ids,
            floating_point_ids: floating_point_ids,
            function_ids: function_ids,
            data: data
        });
    }

    pub fn write_to(&self, path: &str) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(&file);

        write_u32(&mut writer, self.string_ids.len() as u32);
        write_u32(&mut writer, self.integer_ids.len() as u32);
        write_u32(&mut writer, self.floating_point_ids.len() as u32);
        write_u32(&mut writer, self.function_ids.len() as u32);
        write_u32(&mut writer, self.data.len() as u32);

        for entry in &self.string_ids {
            write_u32(&mut writer, entry.id as u32);
            write_u32(&mut writer, entry.address as u32);
        }
        for entry in &self.integer_ids {
            write_u32(&mut writer, entry.id as u32);
            write_u32(&mut writer, entry.address as u32);
        }
        for entry in &self.floating_point_ids {
            write_u32(&mut writer, entry.id as u32);
            write_u32(&mut writer, entry.address as u32);
        }
        for entry in &self.function_ids {
            write_u32(&mut writer, entry.id as u32);
            write_u32(&mut writer, entry.address as u32);
        }

        writer.write(&self.data);
    }

    pub fn get_string(&self, id: u32) -> &str {
        let address = &self.string_ids[id as usize].address;



        return "";
    }

    pub fn add_string(&mut self, string: &str) -> u32 {
        let address = self.data.len() as u32;
        let data = string.as_bytes();
        let length = data.len() as u32;
        let length_data: [u8; 4] = unsafe { transmute(length.to_be()) };

        self.data.extend(length_data.iter().cloned());
        self.data.extend(data.iter().cloned());

        let id = self.string_ids.len() as u32;
        self.string_ids.push(IdToAddressRow { id: id, address: address });

        return id;
    }
}

#[inline(always)]
fn read_u32<T: Read>(reader: &mut BufReader<T>) -> u32 {
    let mut buffer: [u8; 4] = [0;4];
    reader.read_exact(&mut buffer);
    let value: u32 = unsafe { transmute(buffer) };
    return value;
}

#[inline(always)]
fn write_u32<T: Write>(writer: &mut BufWriter<T>, value: u32) {
    let bytes: [u8; 4] = unsafe { transmute(value) };
    writer.write(&bytes);
}
