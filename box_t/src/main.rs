// рекурсивный тип 
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

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
    
}
