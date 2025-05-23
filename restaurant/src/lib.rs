mod front_of_house;

// use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist; // так лучше не делать, так как непонятно потом откуда в итоге берется функция
use std::collections::HashMap; // Но подключать структуры таким образом правильно

use std::fmt::Result;
use std::io::Result as IoResult; // пример альяса, как задать своё имя при импорте
// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

pub use crate::front_of_house::hosting; // реэкспорт элементов

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // это абсолютная ссылка, она наичнается с корневого каталога
    front_of_house::hosting::add_to_waitlist(); // это относительная ссылка, мы находимся с front_of_house на одном уровне и с него начинается отсчет
    hosting::add_to_waitlist(); // такой вызов становится возможным благодаря use
    add_to_waitlist();  // так лучше не делать, так как непонятно потом откуда в итоге берется функция

    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // будет ошибка, так как свойство не публичное

    // enum всегда целиком становится публичным, структура и прочее становится публичным по частям, см. выше
    let order1 = back_of_house::Appetizer::Salat;
    let order2 = back_of_house::Appetizer::Soup;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // обращение к элементу на уровень выше (в родительском)
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peachs"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salat,
    }
}

// вложенные импорты
// --snip--
// use std::cmp::Ordering;
// use std::io;
// --snip--

// --snip--
use std::{cmp::Ordering, io}; // так ипорт из двух строк и больше становится более лаконичным в одну
// --snip--

// ------------------------------------

// use std::io;
// use std::io::Write;

// use std::io::{self, Write}; // импорт себя и вложенного элемента

// -----------------------------------
use std::collections::*; // импортировать весь модуль, все что в нем есть
