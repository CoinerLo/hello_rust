use std::io;
use crate::game::Game;
use crate::ship::ShootResult;

mod ship;
mod board;
mod game;

fn main() {
    let mut game = Game::new();

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

fn parse_coordinates(input: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = input.chars().collect();
    if chars.len() != 2 {
        return None;
    }
    let row = match chars[0] {
        'A'..='J' => chars[0] as usize - 'A' as usize,
        _ => return None,
    };
    let col = chars[1].to_digit(10)? as usize - 1;
    Some((row, col))
}
