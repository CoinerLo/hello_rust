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

    
}


fn takes_ownership(some_string: String) {
    println!("{some_string}");
}