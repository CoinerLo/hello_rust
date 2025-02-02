// Комментарии к документации
// Поддерживается Markdown

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = build_profile::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

// cargo doc --open

// Комментарии к документации как тесты
// cargo test
// Запускает примеры кода из документации как тесты


