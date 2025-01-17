fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = [34, 50, 25, 100, 69];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = ['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let _integer = Point { x: 1, y: 2 }; // Point<i32>
    let _float = Point { x: 1.0, y: 1.8 }; // Point<f64>

    let _integer_and_float = Point2 { x: 1, y: 2.8 };
    let _both_integer = Point2 { x: 3, y: 6 };
    let _both_float = Point2 { x: 2.2, y: 5.5 };

    
}
