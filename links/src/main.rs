fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{y}");
    x
}

fn main() {
    let x = "x";
    let y = "y";
    let z = longest(x, y);
    println!("x = {}, z = {}", x, z);
}
