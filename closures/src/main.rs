use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway (&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    // Тема замыканий
    // Выведение и анотация типов замыканий
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn  add_one_v1 (x: u32) -> u32 { x + 1 }                         // обычная функция 
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // замыкание с полной анотацией типов
    // let add_one_v3 = |x| { x + 1 }; // тоже валидный вариант, но будет подсвечивать ошибкой выведения типа, до первого применения, когда тип сможет вывестись автоматически
    // let add_one_v4 = |x| x + 1;     // самый минималистичный вариант, тип выведется после первого использования, зависимо от переданных данных

    // захват ссылок или передача владения
    // первый вариант - захват неизменяемых ссылок
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    
}
