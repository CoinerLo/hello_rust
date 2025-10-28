// use std::io;
// use crate::game::{Game, RandomShotStrategy};
// use crate::ship::ShootResult;
// use crate::board::{parse_coordinates, ManualShipPlacer, AutoShipPlacer, ShipPlacer};
// use eframe::App;

// mod ship;
// mod board;
// mod game;

// fn main() {
//     println!("Добро пожаловать в игру 'Морской бой'!");
//     println!("Выберите режим размещения кораблей: 1 - автоматически, 2 - вручную");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let player_placer: Box<dyn ShipPlacer> = match input.trim() {
//         "1" => Box::new(AutoShipPlacer),
//         "2" => Box::new(ManualShipPlacer),
//         _ => {
//             println!("Неверный ввод. Используется автоматический режим.");
//             Box::new(AutoShipPlacer)
//         },
//     };
//     let computer_placer = AutoShipPlacer;
//     let mut game = Game::new(&*player_placer, &computer_placer);

//     println!("Итоговое расположение кораблей:");
//     game.player_board.print_board(false);
//     // game.player_board.draw_board(false);

//     loop {
//         println!("Ваш ход! Введите координаты (например, А5):");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let (row, col) = parse_coordinates(&input.trim()).unwrap();
//         let result = game.player_shoot(row, col);

//         match result {
//             ShootResult::Miss => println!("Промах!"),
//             ShootResult::Hit => println!("Попал!"),
//             ShootResult::Destroy => println!("Корабль уничтожен!"),
//         }

//         if game.check_game_over() {
//             println!("Вы победили!");
//             break;
//         }

//         let result = game.computer_shoot(&RandomShotStrategy);
//         match result {
//             ShootResult::Miss => println!("Компьютер промахнулся!"),
//             ShootResult::Hit => println!("Компьютер попал!"),
//             ShootResult::Destroy => println!("Компьютер уничтожил ваш корабль!"),
//         }

//         if game.check_game_over() {
//             println!("Компьютер победил!");
//             break;
//         }
//     }
// }

use eframe::egui;
use crate::game::{Game, RandomShotStrategy};
use crate::board::{parse_coordinates, ManualShipPlacer, AutoShipPlacer, ShipPlacer};

mod ship;
mod board;
mod game;

struct GameApp {
    game: Option<Game>,
    player_placer: Box<dyn ShipPlacer>,
    computer_placer: Box<dyn ShipPlacer>,
    is_player_turn: bool,
    input_coords: String,
}

impl Default for GameApp {
    fn default() -> Self {
        let player_placer = Box::new(AutoShipPlacer);
        let computer_placer = Box::new(AutoShipPlacer);

        GameApp {
            game: None,
            player_placer,
            computer_placer,
            is_player_turn: true,
            input_coords: String::new(),
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.game.is_none() {
                ui.heading("Добро пожаловать в игру 'Морской бой'!");
                ui.label("Выберите режим размещения кораблей:");
                if ui.button("1 - автоматически").clicked() {
                    self.player_placer = Box::new(AutoShipPlacer);
                    self.start_game();
                }
                if ui.button("2 - вручную").clicked() {
                    self.player_placer = Box::new(ManualShipPlacer);
                    self.start_game();
                }
                return;
            }

            ui.heading("Морской бой");

            let game = self.game.as_mut().unwrap();

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Ваша доска:");
                    game.player_board.draw_board(ui, false);
                });

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label("Доска компьютера:");
                    if let Some((row, col)) = game.computer_board.draw_board(ui, true) {
                        if self.is_player_turn {
                            // Выполняем выстрел по компьютеру
                            let result = game.player_shoot(row, col);
                            match result {
                                crate::ship::ShootResult::Miss => {
                                    println!("Промах!");
                                    // Передаём ход компьютеру
                                    self.is_player_turn = false;
                                }
                                crate::ship::ShootResult::Hit => println!("Попадание!"),
                                crate::ship::ShootResult::Destroy => println!("Корабль уничтожен!"),
                            }
                        }
                    };
                });
            });

            if game.check_game_over() {
                ui.add_space(20.0);
                ui.horizontal_centered(|ui| {
                    ui.heading(if game.player_board.all_ships_destroyed() {
                        "Компьютер победил!"
                    } else {
                        "Вы победили!"
                    });
                });
                return;
            }

            if !self.is_player_turn {
                let shoot_result = game.computer_shoot(&RandomShotStrategy);
                match shoot_result {
                    crate::ship::ShootResult::Miss => {
                        self.is_player_turn = true;
                    },
                    crate::ship::ShootResult::Hit => {}
                    crate::ship::ShootResult::Destroy => {}
                }
            }

            // if self.is_player_turn {
            //     ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            //         ui.label("Ваш ход! Введите координаты (например, A5):");
            //         ui.text_edit_singleline(&mut self.input_coords);
            //         if ui.button("Сделать ход").clicked() {
            //             if let Ok((row, col)) = parse_coordinates(&self.input_coords.trim()) {
            //                 let result = game.player_shoot(row, col);
            //                 match result {
            //                     crate::ship::ShootResult::Miss => println!("Промах!"),
            //                     crate::ship::ShootResult::Hit => println!("Попал!"),
            //                     crate::ship::ShootResult::Destroy => println!("Корабль уничтожен!"),
            //                 }
            //                 self.is_player_turn = false;
            //             } else {
            //                 println!("Неверные координаты!");
            //             }
            //         }
            //     });
            // } else {
            //     let result = game.computer_shoot(&RandomShotStrategy);
            //     match result {
            //         crate::ship::ShootResult::Miss => println!("Компьютер промахнулся!"),
            //         crate::ship::ShootResult::Hit => println!("Компьютер попал!"),
            //         crate::ship::ShootResult::Destroy => println!("Компьютер уничтожил ваш корабль!"),
            //     }
            //     self.is_player_turn = true;
            // }
        });
    }
}

impl GameApp {
    fn start_game(&mut self) {
        let game = Game::new(&*self.player_placer, &*self.computer_placer);
        println!("{:?}", game);

        self.game = Some(game);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([800.0, 500.0]), 
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Морской бой",
        options,
        Box::new(|_cc| Ok(Box::new(GameApp::default()))),
    );
}
