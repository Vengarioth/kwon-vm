pub trait ConstantPool {
    fn fetch_string(&self, string_id: u32) -> Result<&[u8], &'static str>;
    fn fetch_constant(&self, constant_id: u32) -> Result<&[u8], &'static str>;
    fn fetch_function(&self, function_id: u32) -> Result<&[u8], &'static str>;
}
