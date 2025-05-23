use core::fmt;

// псевдонимы типов
type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

type Result<T> = std::result::Result<T, std::io::Error>;

// тип never
// fn bar() -> ! {}
// типа never имеют тикие структуры как loop, panic!, continue

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));
    fn _takes_long_type(_f: Thunk) {}
    fn _returns_long_type() -> Thunk {
        Box::new(|| println!("by"))
    }

    // типы с динамическим размером
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
