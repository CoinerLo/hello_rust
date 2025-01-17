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

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let number_list = [34, 50, 25, 100, 69];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = ['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 1, y: 2 }; // Point<i32>
    let _float = Point { x: 1.0, y: 1.8 }; // Point<f64>

    let integer_and_float = Point2 { x: 1, y: 2.8 };
    let _both_integer = Point2 { x: 3, y: 6 };
    let _both_float = Point2 { x: 2.2, y: 5.5 };
    println!("integer_and_float.x = {}, integer_and_float.y = {}", integer_and_float.x, integer_and_float.y);

    let p = integer.x();
    println!("p.x = {}", integer.x);
    println!("p.y = {}", integer.y);
    println!("p.x() = {p}");

    let special: Point<f32> = Point { x: -1.1, y: 2.2 };
    special.distance_from_origin(); // этот метод доступен только для Point с типом f32

    let p1 = Point3 { x: 5, y: 5.5 };
    let p2 = Point3 { x: 'r', y: 'z' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
