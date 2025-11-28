pub enum Type {
    Integer,
    Text,
    Float,
    Boolean,
}

pub struct Row (str, Type)

pub struct Database {
    memory: Row[]
}

pub fn database(schema) -> Database {
    Database {  }
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
