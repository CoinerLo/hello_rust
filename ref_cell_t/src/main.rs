use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

fn main() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {a:?}");
  println!("b after = {b:?}");
  println!("c after = {c:?}");
}

// множественное изменяемое владение
// не работает для многопоточного кода
// есть риск зацикливания
