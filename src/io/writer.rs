use std::fs::{File, OpenOptions};
use std::io::{Read,BufReader,Write,BufWriter};
use std::mem::transmute;

pub fn write_u32_le<T: Write>(writer: &mut BufWriter<T>, value: u32) {
    let bytes: [u8; 4] = unsafe { transmute(u32::to_le(value)) };
    writer.write(&bytes);
}
