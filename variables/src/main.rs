fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let tup = (500, 6.4, 1);

    let (_w, y, _z) = tup;

    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    println!("five_hundred is : {five_hundred}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let second = a[1];
    println!("second  is: {second}");
}
