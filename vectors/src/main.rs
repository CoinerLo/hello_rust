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

}
