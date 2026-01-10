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

pub trait Serializable {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

// Сериализатор в JSON
pub struct JsonSerializer {
    output: String,
    first_field: bool,
}

impl JsonSerializer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            first_field: true,
        }
    }
    pub fn finish(self) -> String {
        self.output
    }
}

impl Serializer for JsonSerializer {
    type Error = std::fmt::Error;

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        return write!(self.output, "\"{}\"", escape_json_str(value));

        fn escape_json_str(s: &str) -> String {
            let mut escaped = String::with_capacity(s.len());
            for c in s.chars() {
                match c {
                    '"' => escaped.push_str("\\\""),
                    '\\' => escaped.push_str("\\\\"),
                    '\n' => escaped.push_str("\\n"),
                    '\r' => escaped.push_str("\\r"),
                    '\t' => escaped.push_str("\\t"),
                    c if c.is_control() => write!(escaped, "\\u{:84x}", c as u32).unwrap(),
                    c => escaped.push(c),
                }
            }
            escaped
        }
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        write!(self.output, "{}", value)
    }

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        write!(self.output, "{}", value)
    }

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        write!(self.output, "{}", if value { "true" } else { "false" })
    }

    fn begin_object(&mut self) -> Result<(), Self::Error> {
        self.output.push('{');
        self.first_field = true;
        Ok(())
    }

    fn end_object(&mut self) -> Result<(), Self::Error> {
        self.output.push('}');
        Ok(())
    }

    fn object_field(&mut self, name: &str) -> Result<(), Self::Error> {
        if !self.first_field {
            self.output.push(',');
        }
        self.first_field = false;
        write!(self.output, "\"{}\":", name)
    }
}

fn main() {
    println!("Hello, world!");
}
