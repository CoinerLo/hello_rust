use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Disply for Cell {
    fn fmt(&self, f: &mut fmt::Formater<'_>) -> fmt::Result {
        write!(f, "{}", if *self == Cell::Alive { "■" } else { " " })
    }
}

fn main() {
    println!("Hello, world!");
}
