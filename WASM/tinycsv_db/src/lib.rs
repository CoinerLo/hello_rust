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
    pub const Boolean: DataType = DataType:Boolean;
}

pub struct Schema {
    pub fn colums: Vec<String, data_type::DataType>,
    column_index: HashMap<String, usize>,
}

pub struct Row (String, Type);

pub struct Database {
    memory: Vec<Row>,
}

pub fn database(schema: Vec<Row>) -> Database {
    Database { memory: schema }
}

pub fn insert_to(db: &mut Database, row: Row) {
    db.memory.push(row);
}

pub fn find_exact(db: &mut Database, name_col: str, value: ) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
