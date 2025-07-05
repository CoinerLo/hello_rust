use crate::board::Board;
use crate::ship::ShootResult;
use rand::Rng;
use crate::board::ShipPlacer;

pub struct Game {
    pub player_board: Board,
    pub computer_board: Board,
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

    pub fn computer_shoot(&mut self) -> ShootResult {
        let mut rng = rand::rng();
        let row = rng.random_range(0..10);
        let col = rng.random_range(0..10);
        self.player_board.shoot(row, col)
    }

    pub fn check_game_over(&self) -> bool {
        self.player_board.all_ships_destroyed() || self.computer_board.all_ships_destroyed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship::Ship;
    use std::{rc::Rc, cell::RefCell};

    #[test]
    fn test_check_game_over_player_wins() {
        let mut game = Game::new("auto");
        let player_ship = Rc::new(RefCell::new(Ship::new(vec![(0, 0)], 1)));
        let computer_ship = Rc::new(RefCell::new(Ship::new(vec![(1, 1)], 1)));

        let _ = game.player_board.place_ship(player_ship);
        let _ = game.computer_board.place_ship(computer_ship);

        assert!(!game.check_game_over(), "Игра должна быть активна");
        assert_eq!(game.player_shoot(1, 1), ShootResult::Destroy);
        assert!(game.check_game_over(), "Игра должна быть закончена");
    }
}
