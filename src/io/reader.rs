use std::fs::{File, OpenOptions};
use std::io::{Read,BufReader,Write,BufWriter};
use std::mem::transmute;

pub fn read_u32_le<T: Read>(reader: &mut BufReader<T>) -> u32 {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer);
    let value: u32 = unsafe { transmute(buffer) };
    return u32::from_le(value);
}
