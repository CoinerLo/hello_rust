// рекурсивный тип 
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x; // ссылка
    assert_eq!(5, x);
    assert_eq!(5, *y); // разименовывание - переход от ссылки к значению (или переход по ссылке к значению)

    let x1 = 7;
    let y1 = Box::new(x1);
    assert_eq!(7, x1);
    assert_eq!(7, *y1); // поведение аналогичное разименовыванию ссылки - так как Box это и есть ссылка, а если быть точнее - умный указатель на данные в куче

    let x2 = 9;
    let y2 = MyBox(x2);
    assert_eq!(9, x2);
    assert_eq!(9, *y2);

    let m = MyBox::new(String::from("Rust"));
    // Неявное разменовывание необходимое кол-во раз MyBox -> String -> str
    hello(&m);
    //Явно пришлось бы писать так:
    hello(&(*m)[..]);

    // есть еще DerefMut
}

struct MyBox<T>(T);

use std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// типаж Deref отвечает за возможность разименовывания

impl<T> Deref for MyBox<T> {
    type Target = T; // связанный тип для использования в Deref

    fn deref(&self) -> &Self::Target {
        &self.0 // ссылка на значение к которому хочет получить доступ оператор *
    }
}

// Rust заменяет оператор * примененный к умным казателям вызовом y.deref() и затем применяет простое разименовывание *(y.deref())
