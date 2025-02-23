pub trait Iterator<T> {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {

    };
}

fn main() {
    println!("Hello, world!");
}
