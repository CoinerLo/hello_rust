fn main() {
    let v: Vec<i32> = Vec::new(); // аннотация типа при пустом новом векторе
    let v2 = vec![1, 2, 3]; // создание макроса с заданными значениями, тип выведется автоматически

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // обращение к вектору двумя вариантами
    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2]; // здесь если индекс выходит за рамки - программа завершится с паникой
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // векторы хранятся в куче, их длина может меняться динамически, значит элементы могут перемещаться в памяти
    // ссылки на отдальные элементы вектора становятся недействительными при изменении вектора

    // Перебор значений
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50; // ------------------------- оператор разыменовывания * 
        println!("{i}");
    }

    // сохраняем множество разных типов
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
