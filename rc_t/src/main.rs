use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// хранилище можественных ссылок на неизменяемые данные
// данные хранятся пока счетчик больше 0
// при 0 - данные автоматически анулируются благодаря типажу Drop
// множественное владение одними и теми же данными
