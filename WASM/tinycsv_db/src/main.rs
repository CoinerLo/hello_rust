
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DataType {
    Integer,
    Text,
    Float,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Text(String),
    Float(f64),
    Boolean(bool),
}

pub mod schema {
    use super::DataType;

    pub type SchemaRow = Vec<(String, DataType)>;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Schema(pub(crate) SchemaRow);

    pub fn new(columns: SchemaRow) -> Schema {
        Schema(columns)
    }
}

pub mod row {
    use super::Value;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Row(pub(crate) Vec<Value>);

    pub fn new(schema: Vec<Value>) -> Row {
        Row(schema)
    }
}

pub mod database {
    use super::schema::Schema;
    use super::row::Row;
    use super::{DataType, Value, insert_to};

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
                db = Some(new(parse_schema(csv_row)));
            } else if csv_row != "" {
                if let Some(db) = &mut db {
                    let row = parse_row(csv_row, &db.schema);
                    insert_to(db, row);
                }
            }
        }

        db.unwrap()
    }

    fn parse_row(row: &str, schema: &Schema) -> Row {
        let mut data = vec![];
        for (i, col) in row.split(",").enumerate() {
            assert!(i < schema.0.len());

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

    fn parse_schema(row: &str) -> Schema {
        let mut schema = vec![];

        for csv_col in row.split(",") {
            let mut name = "";

            for (a, val) in csv_col.split(":").enumerate() {
                match a {
                    0 => name = val,
                    _ => {
                        schema.push((name.to_string(), match val {
                            "Integer" => DataType::Integer,
                            "Text" => DataType::Text,
                            "Float" => DataType::Float,
                            "Boolean" => DataType::Boolean,
                            _ => unreachable!(),
                        }))
                    }
                }
            }
        }
        Schema(schema)
    }
}

use database::Database;
use row::Row;

fn insert_to(db: &mut Database, row: Row) {
    assert_eq!(db.schema.0.len(), row.0.len());

    for (i, t) in db.schema.0.iter().enumerate() {
        assert!(
            match t.1 {
                DataType::Integer => matches!(row.0[i], Value::Integer(_)),
                DataType::Text => matches!(row.0[i], Value::Text(_)),
                DataType::Float => matches!(row.0[i], Value::Float(_)),
                DataType::Boolean => matches!(row.0[i], Value::Boolean(_)),
            },
            "Неправильный тип данных"
        );
    }

    db.data.push(row);
}

fn find_exact(&Database { ref schema, ref data }: &Database, column: &str, value: &Value) -> Box<[usize]> {
    if let Some(column_i) = schema.0.iter().position(|x| x.0 == column) {
        let mut ids = vec![];

        for (id, row) in data.iter().enumerate() {
            if row.0[column_i] == *value {
                ids.push(id);
            }
        }
        ids.into_boxed_slice()
    } else {
        panic!("Колонка {} не найдена в базе данных", column);
    }
}

fn remove_exact(db: &mut Database, column: &str, value: &Value) {
    for id in find_exact(db, column, value) {
        db.data.remove(id);
    }
}

fn find_contains(&Database { ref schema, ref data }: &Database, column: &str, value: &str) -> Box<[usize]> {
    if let Some(column_i) = schema.0.iter().position(|x| x.0 == column) {
        let mut ids = vec![];

        for (id, row) in data.iter().enumerate() {
            match &row.0[column_i] {
                Value::Text(t) if t.contains(value) => ids.push(id),
                _ => {},
            }
        }

        ids.into_boxed_slice()
    } else {
        panic!("Колонка {} не найдена в базе данных", column);
    }
}

fn to_csv(&Database { ref schema, ref data }: &Database) -> String {
    let mut csv = String::new();

    for (column, t) in &schema.0 {
        csv.push_str(&format!("{}:{:?},", column, t));
    }

    csv.pop();
    csv.push('\n');

    for row in data {
        for value in row.0.iter() {
            match value {
                Value::Integer(i) => csv.push_str(&i.to_string()),
                Value::Text(t) => csv.push_str(&t.as_str()),
                Value::Float(f) => csv.push_str(&f.to_string()),
                Value::Boolean(b) => csv.push_str(&b. to_string()),
            }
            csv.push(',');
        }
        csv.pop();
        csv.push('\n');
    }
    csv
}

fn main() {
    let schema = schema::new(vec![
        ("id".to_string(),       DataType::Integer),
        ("name".to_string(),     DataType::Text),
        ("score".to_string(),    DataType::Float),
        ("active".to_string(),   DataType::Boolean),
    ]);

    let mut db = database::new(schema);

    // Вставка
    insert_to(&mut db, row::new(vec![
        Value::Integer(1),
        Value::Text("Alice".to_string()),
        Value::Float(95.5),
        Value::Boolean(true),
    ]));

    insert_to(&mut db, row::new(vec![
        Value::Integer(2),
        Value::Text("Bob".to_string()),
        Value::Float(60.0),
        Value::Boolean(true),
    ]));

    // Поиск
    let ids = find_exact(&db, "name", &Value::Text("Alice".to_string()));

    assert_eq!(*ids, [0]);

    let contains = find_contains(&db, "name", "lic");

    assert_eq!(*contains, [0]);

    // Сериализация / десериализация
    let csv = to_csv(&db);

    assert_eq!(csv, "\
id:Integer,name:Text,score:Float,active:Boolean
1,Alice,95.5,true
2,Bob,60,true
");

    let db2 = database::from_csv(&csv);

    assert_eq!(db2, db);

    remove_exact(&mut db, "name", &Value::Text("Bob".to_string()));

    assert_eq!(to_csv(&db), "\
id:Integer,name:Text,score:Float,active:Boolean
1,Alice,95.5,true
");

    println!("Все тесты пройдены!");
}
