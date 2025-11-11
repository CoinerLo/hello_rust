fn format_message(name: &str, score: u32, level: u32) -> String {
    format!("Привет, {}! Ваш счёт: {}, уровень: {}.", name, score, level)
}

fn build_greeting(name: &str, suffix: &str) -> String {
    // name.to_owned() + " " + suffix
    let mut result = name.to_string();
    result.push(' ');
    result.push_str(suffix);
    result
}

fn main() {
    println!("{}", format_message("igor", 8, 3));
    println!("{}", build_greeting("igor", "lo"));
}
