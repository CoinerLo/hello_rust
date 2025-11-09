fn saturating_add(a: i32, b: i32) -> i32 {
    a + b
}

fn saturating_sub(a: i32, b: i32) -> i32 {
    a - b
}

fn saturating_mul(a: i32, b: i32) -> i32 {
    a * b
}

fn calculate(a: i32, b: i32, o: char) -> i32 {
    match o {
        '+' => saturating_add(a, b),
        '-' => saturating_sub(a, b),
        '*' => saturating_mul(a, b),
        _ => 0
    }
}

fn main() {
    println!("{}", calculate(1, 3, '+'));
    println!("{}", calculate(1, 3, '-'));
    println!("{}", calculate(1, 3, '*'));
    println!("{}", calculate(1, 3, '/'));
}
