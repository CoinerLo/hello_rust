type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));
    fn _takes_long_type(_f: Thunk) {}
    fn _returns_long_type() -> Thunk {
        Box::new(|| println!("by"))
    }
}
