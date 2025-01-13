use::std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Перезапись данных
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Вставка только в случае отсутствия
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{scores2:?}");

    // Создание нового значения на основе старого
    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map2:?}");
}
