use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) { // ограничение типажа - более подробная запись 9 строки. По сути на 9 строке - синтаксический сахар
    println!("Breaking news! {}", item.summarize());
}

// несколько параметров
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Несколько границ типажей для одного параметра
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summaty + Display>(item: &T) {}

// Множество параметров с множеством границ типажей
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[cfg(test)]
mod tests {

}
