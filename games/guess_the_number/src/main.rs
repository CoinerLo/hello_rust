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
    };

    let (range, max_attempts) = match difficulty {
        1 => (50, 10),
        2 => (100, 7),
        3 => (200, 5),
        _ => {
            println!("Неверный выбор уровня сложности!");
            return;
        }
    };

    println!(
        "Вы выбрали уровень {}. Загадано число от 1 до {}. У вас {} попыток.",
        difficulty, range, max_attempts
    );

    let secret_number = rand::rng().random_range(1..=range);
    let mut attempts = 0;

    loop {
        if attempts >= max_attempts {
            println!("Вы исчерпали все попытки! Загаданное число было: {}", secret_number);
            break;
        }

        println!("Попытка {}/{}. Введите ваше предположение:", attempts + 1, max_attempts);

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
        attempts += 1;
    }
}
