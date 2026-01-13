use std::error::Error;
use std::fmt;

/// Ошибка при создании AsciiString из невалидных данных
#[derive(Debug)]
pub struct AsciiError {
    msg: String,
    cause: Option<Box<dyn Error + 'static>>
}

impl AsciiError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            cause: None,
        }
    }

    fn with_cause(msg: impl Into<String>, cause: impl Error + 'static) -> Self {
        Self {
            msg: msg.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AsciiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as _)
    }
}

/// Владеющая ASCII-строка (байты 0–127)


fn main() {
    // ========================
    // Успешное создание из ASCII-данных
    // ========================
    let hello = {
        let ascii_str: AsciiString = b"Hello, ASCII!".into();
        println!("Создано успешно: {}", ascii_str);
        println!("Длина: {}", ascii_str.len());

        assert_eq!(ascii_str.len(), 13);
        assert!(ascii_str.contains("ASCII"));
        assert_eq!(ascii_str.to_ascii_uppercase(), b"HELLO, ASCII!".into());

        ascii_str
    };

    println!("✓ Успешное создание и базовые методы работают\n");

    // ========================
    // Паника при не-ASCII (обрабатываем безопасно)
    // ========================

    {
        use std::panic;

        let result = panic::catch_unwind(|| {
            let _invalid = AsciiString::new("Привет ☃");
        });

        assert!(result.is_err(), "Ожидалась паника при не-ASCII символах");
        println!("✓ Паника при не-ASCII корректно срабатывает\n");
    }

    // ========================
    // Безопасное создание через TryFrom
    // ========================
    let valid = {
        let result = AsciiString::try_from("valid ascii");
        assert!(result.is_ok(), "Валидная ASCII-строка должна конвертироваться");
        result.unwrap()
    };

    println!("Безопасно создано из ASCII: {}", valid);

    let invalid = {
        let result = AsciiString::try_from("не ascii ☃");
        assert!(result.is_err(), "Не-ASCII строка должна возвращать ошибку");
        result.unwrap_err()
    };
    println!("Ошибка при не-ASCII: {}\n✓ TryFrom работает корректно\n", invalid);

    // ========================
    // Преобразование регистра
    // ========================
    let mixed_case = {
        let original = AsciiString::new("HeLLo WoRLd 123!");
        let lower = original.to_ascii_lowercase();
        let upper = original.to_ascii_uppercase();

        assert_eq!(lower, AsciiString::new("hello world 123!"));
        assert_eq!(upper, AsciiString::new("HELLO WORLD 123!"));

        (original, lower, upper)
    };

    println!("Оригинал: {}", mixed_case.0);
    println!("Нижний регистр: {}", mixed_case.1);
    println!("Верхний регистр: {}", mixed_case.2);
    println!("✓ Преобразование регистра работает\n");

    // ========================
    // Debug и Display
    // ========================
    {
        println!("Display вывод: {}", hello);
        println!("Debug вывод: {:?}", hello);
        println!("✓ Display и Debug реализованы корректно\n");
    }

    println!("🎉 Все тесты успешно пройдены!");
}
