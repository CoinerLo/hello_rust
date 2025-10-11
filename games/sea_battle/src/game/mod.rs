use crate::board::Board;
use crate::ship::ShootResult;
use rand::Rng;
use crate::board::ShipPlacer;
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct Game {
    pub player_board: Board,
    pub computer_board: Board,
}

impl Debug for Game {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.player_board.print_board(false);
        println!("\n");
        self.computer_board.print_board(false);
        Ok(())
    }
}

pub trait ShotStrategy {
    fn choose_shot(&self, board: &Board) -> (usize, usize);
}

pub struct RandomShotStrategy;

impl ShotStrategy for RandomShotStrategy {
    fn choose_shot(&self, board: &Board) -> (usize, usize) {
        let mut rng = rand::rng();
        let row = rng.random_range(0..board.height);
        let col = rng.random_range(0..board.width);
        (row, col)
    }
}

impl Game {
    pub fn new(player_placer: &dyn ShipPlacer, computer_placer: &dyn ShipPlacer) -> Self {
        let mut game = Game {
            player_board: Board::new(10, 10),
            computer_board: Board::new(10, 10),
        };

        if let Err(err) = player_placer.place_ships(&mut game.player_board) {
            panic!("Ошибка при размещении кораблей игрока: {}", err);
        }

        if let Err(err) = computer_placer.place_ships(&mut game.computer_board) {
            panic!("Ошибка при размещении кораблей коспьютера: {}", err);
        }

        game
    }

    pub fn player_shoot(&mut self, row: usize, col: usize) -> ShootResult {
        self.computer_board.shoot(row, col)
    }

    pub fn computer_shoot(&mut self, strategy: &dyn ShotStrategy) -> ShootResult {
        let (row, col) = strategy.choose_shot(&self.player_board);
        self.player_board.shoot(row, col)
    }

    pub fn check_game_over(&self) -> bool {
        self.player_board.all_ships_destroyed() || self.computer_board.all_ships_destroyed()
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Морской бой");

            ui.label("Ваша доска:");
            self.player_board.draw_board(ui, false);

            ui.label("Доска компьютера:");
            self.computer_board.draw_board(ui, true);

            if self.check_game_over() {
                ui.label(if self.player_board.all_ships_destroyed() {
                    "Компьютер победил!"
                } else {
                    "ВЫ победили!"
                });
                return;
            }
        });
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship::Ship;
    use std::{rc::Rc, cell::RefCell};

    struct MockShipPlacer {
        ship: Rc<RefCell<Ship>>,
    }

    impl ShipPlacer for MockShipPlacer {
        fn place_ships(&self, board: &mut Board) -> Result<(), String> {
            board.place_ship(self.ship.clone())
        }
    }

    pub struct MockShotStrategy {
        coords: (usize, usize),
    }

    impl ShotStrategy for MockShotStrategy {
        fn choose_shot(&self, _board: &Board) -> (usize, usize) {
            self.coords
        }
    }

    #[test]
    fn test_check_game_over_player_wins() {
        let player_ship = Rc::new(RefCell::new(Ship::new(vec![(0, 0)], 1)));
        let computer_ship = Rc::new(RefCell::new(Ship::new(vec![(1, 1)], 1)));

        let player_placer = MockShipPlacer { ship: player_ship };
        let computer_placer = MockShipPlacer { ship: computer_ship };

        let mut game = Game::new(&player_placer, &computer_placer);

        assert!(!game.check_game_over(), "Игра должна быть активна");
        assert_eq!(game.player_shoot(1, 1), ShootResult::Destroy);
        assert!(game.check_game_over(), "Игра должна быть закончена");
    }

    #[test]
    fn test_check_game_over_computer_wins() {
        let player_ship = Rc::new(RefCell::new(Ship::new(vec![(0, 0)], 1)));
        let computer_ship = Rc::new(RefCell::new(Ship::new(vec![(1, 1)], 1)));

        let player_placer = MockShipPlacer { ship: player_ship };
        let computer_placer = MockShipPlacer { ship: computer_ship };

        let mut game = Game::new(&player_placer, &computer_placer);

        assert!(!game.check_game_over(), "Игра должна быть активна");

        let strategy = MockShotStrategy{ coords: (0, 0) };
        assert_eq!(game.computer_shoot(&strategy), ShootResult::Destroy);

        assert!(game.check_game_over(), "Игра должна быть закончена");
    }
}
