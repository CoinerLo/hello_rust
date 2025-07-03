use std::io;
use crate::game::Game;
use crate::ship::ShootResult;
use crate::board::parse_coordinates;

mod ship;
mod board;
mod game;

fn main() {
    println!("Добро пожаловать в игру 'Морской бой'!");
    println!("Выберите режим размещения кораблей: 1 - автоматически, 2 - вручную");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mode = match input.trim() {
        "1" => "auto",
        "2" => "manual",
        _ => {
            println!("Неверный ввод. Используется автоматический режим.");
            "auto"
        },
    };

    let mut game = Game::new(mode);

    loop {
        println!("Ваш ход! Введите координаты (например, А5):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let (row, col) = parse_coordinates(&input.trim()).unwrap();
        let result = game.player_shoot(row, col);

        match result {
            ShootResult::Miss => println!("Промах!"),
            ShootResult::Hit => println!("Попал!"),
            ShootResult::Destroy => println!("Корабль уничтожен!"),
        }

        if game.check_game_over() {
            println!("Вы победили!");
            break;
        }

        let result = game.computer_shoot();
        match result {
            ShootResult::Miss => println!("Компьютер промахнулся!"),
            ShootResult::Hit => println!("Компьютер попал!"),
            ShootResult::Destroy => println!("Компьютер уничтожил ваш корабль!"),
        }

        if game.check_game_over() {
            println!("Компьютер победил!");
            break;
        }
    }
}
