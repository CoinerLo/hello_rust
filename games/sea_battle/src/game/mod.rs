use crate::board::Board;
use crate::ship::ShootResult;
use rand::Rng;

pub struct Game {
    pub player_board: Board,
    pub computer_board: Board,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_board: Board::new(10, 10),
            computer_board: Board::new(10, 10),
            game_over: false,
        }
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
