use eframe::egui;
use crate::game::{Game, RandomShotStrategy};
use crate::board::{ManualShipPlacer, AutoShipPlacer, ShipPlacer};

mod ship;
mod board;
mod game;

struct GameApp {
    game: Option<Game>,
    player_placer: Box<dyn ShipPlacer>,
    computer_placer: Box<dyn ShipPlacer>,
    is_player_turn: bool,
    last_shot_result: Option<String>,
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
            last_shot_result: None,
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
                                    self.last_shot_result = Some("Промах!".to_string());
                                    // Передаём ход компьютеру
                                    self.is_player_turn = false;
                                }
                                crate::ship::ShootResult::Hit => {
                                    println!("Попадание!");
                                    self.last_shot_result = Some("Попадание!".to_string());
                                }
                                crate::ship::ShootResult::Destroy => {
                                    println!("Корабль уничтожен!");
                                    self.last_shot_result = Some("Корабль уничтожен!".to_string());
                                }
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
            } else if let Some(result) = &self.last_shot_result {
                ui.add_space(50.0);
                ui.heading(format!("Результат выстрела: {}", result));
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
