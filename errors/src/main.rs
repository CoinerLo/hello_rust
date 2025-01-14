use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            },
        },
    };

    // Более лаконичный вариант обработки ошибок
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap
    let greeting_file = File::open("hello.txt").unwrap();

    // expect
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
