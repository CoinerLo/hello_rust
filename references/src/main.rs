fn main() {
    let mut s = String::from("hello world");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");
    // println!("{r2}"); // Здесь переменные r1 и r2 уже не существуют, владение перешло к r3

    let z = first_word(&s);
    println!("z = {z}");

    let my_string = String::from("hello world");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn f_w(s: &str) -> &str {
    
}
