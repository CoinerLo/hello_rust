use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
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
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap
    let _greeting_file = File::open("hello.txt").unwrap();

    // expect
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    
    let _ = read_username_from_file();
    let _ = read_username_from_file_with_operator();
    let _ = read_username_from_file_with_operatorv2();
}

// Проброс ошибок

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Оператор ?
fn read_username_from_file_with_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_with_operatorv2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("text.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
