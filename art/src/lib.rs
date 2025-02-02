//! # Art
//! 
//! A library for modeling artistic concepts.

// Обзедоступный реэкспорт

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary color according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to crate a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match c1 {
            PrimaryColor::Blue => SecondaryColor::Green,
            PrimaryColor::Red => SecondaryColor::Orange,
            PrimaryColor::Yellow => SecondaryColor::Purple,
        }
    }
}

// cargo doc --open

// Настройка учетной записи
// cargo login <ключ API из ЛК по адресу https://crates.io/me/>

// Публикация своего пакета в общий репозиторий

// cargo publish

// [package]
// name = "my_awesome_name"
// version = "0.1.0"
// edition = "2021"
// description = "A fun game where you guess what number the computer has chosen."
// license = "MIT OR Apache-2.0"

// Вычеркнуть версию крейта из общей библиотеки, что бы новые проекты не могли её устанавливать
// cargo yank --vers 1.0.1

// Отменить выламывание и вернуть версию к установке
// cargo yank --vers 1.0.1 --undo
