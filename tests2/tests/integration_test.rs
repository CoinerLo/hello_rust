// интеграционные тесты
use tests2::add_two;

mod common;

#[test]
fn it_adds_two() {
  common::setup(); // common не считается тестом и подключается как обычный модуль

  let result = add_two(2);
  assert_eq!(result, 4);
}

// запуск тестов из файла
// cargo test --test integration_test
