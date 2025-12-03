use std::collections::HashMap;

pub mod data_type {
    #[derive(Debug, Clone, PartialEq)]
    pub enum DataType {
        Integer,
        Text,
        Float,
        Boolean,
    }

    pub const Integer: DataType = DataType::Integer;
    pub const Text: DataType = DataType::Text;
    pub const Float: DataType = DataType::Float;
    pub const Boolean: DataType = DataType::Boolean;
}

pub struct Schema {
    pub columns: Vec<(String, data_type::DataType)>,
    column_index: HashMap<String, usize>,
}

impl Schema {
    pub fn new(columns: Vec<(String, data_type::DataType)>) -> Self {
        let column_index = columns
            .iter()
            .enumerate()
            .map(|(i, (name, _))| (name.clone(), i))
            .collect();
        Schema { columns, column_index }
    }

    pub fn column_type(&self, name: &str) -> Option<&data_type::DataType> {
        self.column_index
            .get(name)
            .map(|&i| &self.columns[i].1)
    }
}

pub mod schema {
    use super::{data_type, Schema};
    pub fn new(columns: Vec<(String, data_type::DataType)>) -> Schema {
        Schema::new(columns)
    }
}

pub mod value {
    #[derive(Debug, Clone)]
    pub enum Value {
        Integer(i64),
        Text(String),
        Float(f64),
        Boolean(bool),
    }

    pub fn Integer(v: i64) -> Value { Value::Integer(v) }
    pub fn Text(v: String) -> Value { Value::Text(v) }
    pub fn Float(v: f64) -> Value { Value::Float(v) }
    pub fn Boolean(v: bool) -> Value { Value::Boolean(v) }
}

pub use value::Value;

pub mod row {
    use super::Value;

    #[derive(Debug, Clone)]
    pub struct Row {
        pub values: Vec<Value>,
    }

    impl Row {
        pub fn new(values: Vec<Value>) -> Self {
            Row { values }
        }
    }

    pub fn new(values: Vec<Value>) -> Row {
        Row::new(values)
    }
}

pub mod database {
    use super::Schema;
    pub struct Database {
        pub schema: Schema,
    }

    impl Database {
        pub fn new(schema: Schema) -> Self {
            Database { schema }
        }
    }

    pub fn new(schema: Schema) -> Database {
        Database::new(schema)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // let schema = schema::new(vec![
    //     ("id".to_string(),       data_type::Integer),
    //     ("name".to_string(),     data_type::Text),
    //     ("score".to_string(),    data_type::Float),
    //     ("active".to_string(),   data_type::Boolean),
    // ]);

    // let mut db = database::new(schema);

    // // Вставка
    // insert_to(&mut db, row::new(vec![
    //     value::Integer(1),
    //     value::Text("Alice".to_string()),
    //     value::Float(95.5),
    //     value::Boolean(true),
    // ]));

    // // Поиск
    // let ids = find_exact(&db, "name", &value::Text("Alice".to_string()));
    // let contains = find_contains(&db, "name", "lic");

    // // Сериализация / десериализация
    // let csv = to_csv(&db);
    // let db2 = database::from_csv(&csv);
}
