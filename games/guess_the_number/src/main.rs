use rand::Rng;
use std::io;

fn main() {
    println!("Добро пожаловать в игру 'Угадай число'!");

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
    }
}
