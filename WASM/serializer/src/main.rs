use std::fmt::Write;

pub trait Serializer {
    type Error;

    fn begin_object(&mut self) -> Result<(), Self::Error>;

    fn end_object(&mut self) -> Result<(), Self::Error>;

    fn object_field(&mut self, name: &str) -> Result<(), Self::Error>;

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error>;

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error>;

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error>;

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error>;

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        self.serialize_u64(value as u64)
    }
}



fn main() {
    println!("Hello, world!");
}
