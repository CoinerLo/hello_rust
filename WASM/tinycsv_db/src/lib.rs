use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum DataType {
    Integer,
    Text,
    Float,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Integer(i64),
    Text(String),
    Float(f64),
    Boolean(bool),
}

pub mod schema {
    use super::DataType;

    pub type SchemaRow = Vec<(String, DataType)>

    #[derive(Debug, Clone, PartialEq)]
    pub struct Schema(pub(crate) SchemaRow);

    pub fn new(columns: SchemaRow) -> Schema {
        Schema(columns)
    }
}

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

        // pub fn from_csv(csv: &str) -> Self {
            
        //     Database { schema }
        // }
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
