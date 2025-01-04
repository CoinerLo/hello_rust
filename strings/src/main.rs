fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    {
        let mut z = String::from("Hello");
        z.push_str(", world");

    }
    // z.push_str(", new world");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let st = String::from("Hello");
    takes_ownership(st);
    // println!("{st}"); // Не сработает, так как владение передано в функцию и st более не валидная переменная


    let w = 5;
    makes_copy(w);
    println!("{}", w); // Сработает, так как число передаётся копированием

    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("{s5}");
    let (s6, len6) = calculate_length(s3);
    println!("s6 = {s6}, len6 = {len6}");
}


fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    String::from("world")
}

fn takes_and_gives_back(st: String) -> String {
    st
}

fn calculate_length(a_string: String) -> (String, usize) {
    let legth = a_string.len();
    (a_string, legth)
}
