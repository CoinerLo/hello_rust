pub enum Type {
    Integer,
    Text,
    Float,
    Boolean,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
