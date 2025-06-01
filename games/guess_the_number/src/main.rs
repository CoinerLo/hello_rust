use rand::Rng;

fn main() {
    println!("Добро пожаловать в игру 'Угадай число'!");

    let secret_number = rand::rng().random_range(1..=100);
}
