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

fn main() {
    let x = "x";
    let y = "y";
    let z = longest(x, y);
    println!("x = {}, z = {}", x, z);

    let t = "t";
    let p = "p";
    let w = longest_first(t, p);
    println!("w = {w}");
}
