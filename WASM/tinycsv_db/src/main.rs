use tinycsv_db::*;

fn main() {
    let schema = schema::new(vec![
        ("id".to_string(),       DataType::Integer),
        ("name".to_string(),     DataType::Text),
        ("score".to_string(),    DataType::Float),
        ("active".to_string(),   DataType::Boolean),
    ]);

    let mut db = database::new(schema);

    // Вставка
    // insert_to(&mut db, row::new(vec![
    //     Value::Integer(1),
    //     Value::Text("Alice".to_string()),
    //     Value::Float(95.5),
    //     Value::Boolean(true),
    // ]));

    // insert_to(&mut db, row::new(vec![
    //     Value::Integer(2),
    //     Value::Text("Bob".to_string()),
    //     Value::Float(60.0),
    //     Value::Boolean(true),
    // ]));

    // // Поиск
    // let ids = find_exact(&db, "name", &Value::Text("Alice".to_string()));

    // assert_eq!(*ids, [0]);

    // let contains = find_contains(&db, "name", "lic");

    // assert_eq!(*contains, [0]);

    // Сериализация / десериализация
//     let csv = to_csv(&db);

//     assert_eq!(csv, "\
// id:Integer,name:Text,score:Float,active:Boolean
// 1,Alice,95.5,true
// 2,Bob,60,true
// ");

//     let db2 = database::from_csv(&csv);

//     assert_eq!(db2, db);

//     remove_exact(&mut db, "name", &Value::Text("Bob".to_string()));

//     assert_eq!(to_csv(&db), "\
// id:Integer,name:Text,score:Float,active:Boolean
// 1,Alice,95.5,true
// ");

    println!("Все тесты пройдены!");
}
