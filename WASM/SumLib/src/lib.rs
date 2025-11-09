//! Сумматор

/// Суммируем два числа!!!
/// Вот такие примеры у нас
/// Базовое использование:
/// ```
/// let result = SumLib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Работа с отрицательными числами:
/// ```
/// let result = SumLib::add(-5, 10);
/// assert_eq!(result, 5);
/// ```
///
/// Можно использовать в выражениях:
/// ```
/// use SumLib::add;
/// if add(1, 2) == 3 {
///     println!("Математика работает!");
/// }
/// ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// cargo test --doc
// cargo doc --open
