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

    #[derive(Debug, Clone, PartialEq)]
    pub struct Row(pub(crate) Vec<Value>)

    pub fn new(schema: Vec<Value>) -> Row {
        Row(schema)
    }
}

pub mod database {
    use super::schema::Schema;
    use super::row::Row;
    use super::{DataType, Value}

    #[derive(Debug, Clone, PartialEq)]
    pub struct Database {
        pub(crate) schema: Schema,
        pub(crate) data: Vec<Row>
    }

    pub fn new(schema: Schema) -> Database {
        Database { schema, data: vec![] }
    }

    pub fn from_csv(csv: &str) -> Database {
        let mut db = None;

        for (i, csv_row) in csv.split("\n").enumerate() {
            if i == 0 {
                db == Some(new(parse_schema(csv_row)));
            } else if csv_row != "" {
                if let Some(db) = &mut db {
                    let row = parse_row(csv_row, &db_schema);
                    insert_to(db, row);
                }
            }
        }

        db.unwrap();
    }

    fn parse_row(row: &str, schema: Schema) -> Row {
        let mut data = vec![];
        for (i, col) in row.split(",").enumerate() {
            assert(i < schema.0.len());

            match schema.0[i].1 {
                DataType::Integer => {
                    data.push(Value::Integer(col.parse::<i64>().unwrap()));
                }
                DataType::Text => {
                    data.push(Value::Text(col.to_string()));
                }
                DataType::Float => {
                    data.push(Value::Float(col.parse::<f64>().unwrap()));
                }
                DataType::Boolean => {
                    data.push(Value::Boolean(col == "true"));
                }
            }
        }
        Row(data)
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
