use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}

fn main() {
  // Пример зацикливания
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());
}

// множественное изменяемое владение
// не работает для многопоточного кода
// есть риск зацикливания
