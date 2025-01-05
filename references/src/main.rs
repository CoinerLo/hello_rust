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

    let my_string = "hello world";
    let f = f_w(my_string);
    let word1 = f_w(&my_string[0..6]);
    let word2 = f_w(&my_string[..]);
    println!("f = {f}");
    println!("word1 = {word1}");
    println!("word2 = {word2}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, a);
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
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
