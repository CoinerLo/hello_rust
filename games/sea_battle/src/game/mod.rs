use crate::board::Board;
use crate::ship::ShootResult;
use rand::Rng;

pub struct Game {
    pub player_board: Board,
    pub computer_board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            player_board: Board::new(10, 10),
            computer_board: Board::new(10, 10),
        };

        println!("Выберите режим размещения кораблей: 1 - автоматически, 2 - вручную");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                if game.player_board.place_ships_randomly().is_err() {
                    println!("Не удалось разместить корабли автоматически. Перегенерация...");
                    loop {
                        if game.player_board.place_ships_randomly().is_ok() {
                            break;
                        }
                    }
                }
            }
            "2" => {
                if place_ships_manually(&mut game.player_board).is_err() {
                    panic!("Не удалось разместить корабли вручную");
                }
            }
            _ => panic!("Неверный выбор режима размещения"),
        }

        loop {
            if game.computer_board.place_ships_randomly().is_ok() {
                break;
            }
            // перезапуск генерации досок
            println!("Перегенерация досок компьютера...");
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
