use std::io;
use crate::game::{Game, RandomShotStrategy};
use crate::ship::ShootResult;
use crate::board::{parse_coordinates, ManualShipPlacer, AutoShipPlacer, ShipPlacer};

mod ship;
mod board;
mod game;

fn main() {
    println!("Добро пожаловать в игру 'Морской бой'!");
    println!("Выберите режим размещения кораблей: 1 - автоматически, 2 - вручную");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let player_placer: Box<dyn ShipPlacer> = match input.trim() {
        "1" => Box::new(AutoShipPlacer),
        "2" => Box::new(ManualShipPlacer),
        _ => {
            println!("Неверный ввод. Используется автоматический режим.");
            Box::new(AutoShipPlacer)
        },
    };
    let computer_placer = AutoShipPlacer;
    let mut game = Game::new(&*player_placer, &computer_placer);

    println!("Итоговое расположение кораблей:");
    game.player_board.print_board(false);

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

        let result = game.computer_shoot(&RandomShotStrategy);
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
