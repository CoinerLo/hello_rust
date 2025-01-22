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
    part: &'a str,
}

// три правила неявного вывода времени жизни ссылки
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
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
        part: first_sentence,
    };

    // статическое время жизни - все время работы программы
    let s: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
