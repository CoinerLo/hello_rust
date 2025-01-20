use std::fmt::Display;

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
pub fn notify(item: &(impl Summary + Display)) {}

#[cfg(test)]
mod tests {

}
