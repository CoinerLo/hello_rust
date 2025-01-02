fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = plus_one(y);
    println!("{z}");

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of num is: {num}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

