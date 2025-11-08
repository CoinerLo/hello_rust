fn fizz_buzz(num: i32) {
    let is_three = num % 3 == 0;
    let is_five = num % 5 == 0;
    if is_three && is_five {
        println!("FizzBuzz");
    } else if is_three {
        println!("Fizz");
    } else if is_five {
        println!("Buzz");
    } else {
        println!("{}", num);
    }
}

fn main() {
    fizz_buzz(9);
    fizz_buzz(25);
    fizz_buzz(15);
    fizz_buzz(13);
}
