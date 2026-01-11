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

// Сериализатор в Debug
pub struct DebugSerializer {
    output: String,
    first_field: bool,
}

impl DebugSerializer {
    pub fn new(type_name: &str) -> Self {
        let mut s = Self {
            output: String::new(),
            first_field: true,
        };
        write!(s.output, "{} {{ ", type_name).unwrap();
        s
    }
    pub fn finish(self) -> String {
        let mut out = self.output;
        out.push_str(" }");
        out
    }
}

impl Serializer for DebugSerializer {
    type Error = std::fmt::Error;

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        write!(self.output, "\"{}\"", value)
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        write!(self.output, "{}", value)
    }

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        write!(self.output, "{}", value)
    }

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        write!(self.output, "{}", value)
    }

    fn begin_object(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end_object(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn object_field(&mut self, name: &str) -> Result<(), Self::Error> {
        if !self.first_field {
            self.output.push_str(", ");
        }
        self.first_field = false;
        write!(self.output, "{}: ", name)
    }
}

// Универсальные функции сериализации
pub fn to_json<T: Serializable>(value: &T) -> Result<String, std::fmt::Error> {
    let mut ser = JsonSerializer::new();
    value.serialize(&mut ser)?;
    Ok(ser.finish())
}

pub fn to_debug<T: Serializable>(type_name:&str, value: &T) -> Result<String, std::fmt::Error> {
    let mut ser = DebugSerializer::new(type_name);
    value.serialize(&mut ser)?;
    Ok(ser.finish())
}

// Пример пользовательского типа
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

impl Serializable for Person {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.begin_object()?;
        serializer.object_field("name")?;
        serializer.serialize_str(&self.name)?;

        serializer.object_field("age")?;
        serializer.serialize_u32(self.age)?;

        serializer.object_field("is_student")?;
        serializer.serialize_bool(self.is_student)?;
        serializer.end_object()?;

        Ok(())
    }
}

fn main() {
    {
        let person = Person {
            name: "Алексей".to_string(),
            age: 25,
            is_student: true,
        };

        let json = to_json(&person).unwrap();

        let debug = to_debug("Person", &person).unwrap();

        // Тест JSON
        assert_eq!(json, r#"{"name":"Алексей","age":25,"is_student":true}"#);
        println!("✓ JSON тест пройден");

        // Тест Debug
        assert_eq!(debug, "Person { name: \"Алексей\", age: 25, is_student: true }");
        println!("✓ Debug тест пройден");
    }

    {
        // Тест с экранированием
        let person = Person {
            name: r#"О"Коннор"#.to_string(),
            age: 30,
            is_student: false,
        };

        let json = to_json(&person).unwrap();
        assert_eq!(json, r#"{"name":"О\"Коннор","age":30,"is_student":false}"#);
        println!("✓ Тест экранирования пройден");
    }


    {
        // Тест с управляющим символом (например, \n)
        let person = Person {
            name: "Привет\nмир".to_string(),
            age: 42,
            is_student: true,
        };

        let json = to_json(&person).unwrap();
        assert_eq!(json, r#"{"name":"Привет\nмир","age":42,"is_student":true}"#);
        println!("✓ Тест с \\n пройден");
    }

    println!("\nВсе тесты успешно пройдены!");
}
