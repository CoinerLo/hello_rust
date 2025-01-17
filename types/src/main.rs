fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = [34, 50, 25, 100, 69];
    let result = largest(&number_list);
    println!("{result}");

    let char_list = ['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("{result}");
}
