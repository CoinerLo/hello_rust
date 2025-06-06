use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Добро пожаловать в игру 'Угадай число'!");

    let (range, max_attempts) = choose_difficulty();

    println!(
        "Загадано число от 1 до {}. У вас {} попыток.",
        range, max_attempts
    );

    let secret_number = generate_random_number(range);

    play_game(secret_number, max_attempts);
}

fn choose_difficulty() -> (u32, u32) {
    println!("Выберите уровень сложности:");
    println!("1. Легкий (Диапазон 1-50 включительно, 10 попыток)");
    println!("2. Средний (Диапазон 1-100 включительно, 7 попыток)");
    println!("3. Сложный (Диапазон 1-200 включительно, 5 попыток)");

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Выберите 1, 2 или 3");

    match difficulty.trim().parse::<u32>() {
        Ok(1) => (50, 10),
        Ok(2) => (100, 7),
        Ok(3) => (200, 5),
        _ => {
            println!("Неверный выбор уровня сложности!");
            std::process::exit(1);
        }
    }
}

fn generate_random_number(range: u32) -> u32 {
    rand::rng().random_range(1..=range)
}

fn play_game(secret_number: u32, max_attempts: u32) {
    let mut attempts = 0;

    loop {
        if attempts >= max_attempts {
            println!("Вы исчерпали все попытки! Загаданное число было: {}", secret_number);
            break;
        }

        println!("Попытка {}/{}. Введите ваше предположение:", attempts + 1, max_attempts);

        let guess = read_user_input();

        println!("Вы ввели: {}", guess);

        match compare_numbers(guess, secret_number) {
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

fn read_user_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Неудалось прочитать строку!");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(e) => {
                println!("Ошибка ввода: {}", e);
            },
        }
    }
}

fn compare_numbers(guess: u32, secret_number: u32) -> Ordering {
    guess.cmp(&secret_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_numbers() {
        assert_eq!(compare_numbers(10, 20), Ordering::Less);
        assert_eq!(compare_numbers(30, 20), Ordering::Greater);
        assert_eq!(compare_numbers(25, 25), Ordering::Equal);
    }

    #[test]
    fn test_generate_random_number() {
        let range = 100;
        let number = generate_random_number(range);
        assert!(number >= 1 && number <= range);
    }

    #[test]
    fn test_read_user_input_valid() {
        let input = "42\n";
        let mut mock_input = input.as_bytes();
        let result = resd_user_input_from_mock(&mut mock_input);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_read_user_input_invalid_then_valid() {
        let input = "abc\n47\n";
        let mut mock_input = input.as_bytes();
        let result = resd_user_input_from_mock(&mut mock_input);
        assert_eq!(result, 47);
    }

    fn resd_user_input_from_mock(input: &mut dyn io::BufRead) -> u32 {
        loop {
            let mut buffer = String::new();
            input.read_line(&mut buffer).unwrap();
            match buffer.trim().parse::<u32>() {
                Ok(num) => return num,
                Err(_) => continue,
            }
        }
    }
}
