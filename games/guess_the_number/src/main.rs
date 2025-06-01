use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Добро пожаловать в игру 'Угадай число'!");

    println!("Выберите уровень сложности:");
    println!("1. Легкий (Диапазон 1-50 включительно, 10 попыток)");
    println!("2. Средний (Диапазон 1-100 включительно, 7 попыток)");
    println!("3. Сложный (Диапазон 1-200 включительно, 5 попыток)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Выберите 1, 2 или 3");

    let difficulty: u32 = match difficulty.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Необходимо выбрать 1, 2 или 3");
            return;
        }
    }

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Пожалуйста, введите ваше предположение:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Неудалось прочитать строку!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Ошибка ввода: {}", e);
                continue;
            },
        };

        println!("Вы ввели: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            }
        }
    }
}
