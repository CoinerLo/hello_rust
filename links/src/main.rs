fn longest_first<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{y}");
    x
}

// время жизни ссылки в структуре
struct ImportantExcerpt<'a> {
    _part: &'a str,
}

fn main() {
    let x = "x";
    let y = "y";
    let z = longest(x, y);
    println!("x = {}, z = {}", x, z);

    let t = "t";
    let p = "p";
    let w = longest_first(t, p);
    println!("w = {w}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        _part: first_sentence,
    };


}
