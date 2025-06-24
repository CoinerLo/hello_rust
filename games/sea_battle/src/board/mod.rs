use crate::ship::{Ship, ShootResult};
use rand::Rng;

pub struct Board {
    pub cells: Vec<Vec<Option<Ship>>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![vec![None; width]; height],
            width,
            height,
        }
    }

    pub fn place_ships_randomly(&mut self)-> Result<(), String> {
        const MAX_ATTEMPTS: usize = 1000;

        let chips_to_place = vec![
            (4, 1), // 1 четырёхпалубный корабль
            (3, 2), // 2 трёхпалубных корабля
            (2, 3), // 3 двухпалубных корабля
            (1, 4), // 4 однопалубных корабля
        ];

        for &(size, count) in &chips_to_place {
            for _ in 0..count {
                let mut attempts = 0;
                while !self.place_random_ship(size) {
                    attempts += 1;
                    if attempts > MAX_ATTEMPTS {
                        return Err("Не удалось разместить все корабли".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn place_random_ship(&mut self, size: usize) -> bool {
        let mut rng = rand::rng();
        let vertical = rng.random_bool(0.5);

        let max_row = if vertical { self.height - size } else { self.height };
        let max_col = if vertical { self.width } else { self.width - size };

        if max_row == 0 || max_col == 0 {
            return false; // не хватает места
        }

        let row = rng.random_range(0..max_row);
        let col = rng.random_range(0..max_col);

        let coords: Vec<(usize, usize)> = if vertical {
            (row..row + size).map(|r| (r, col)).collect()
        } else {
            (col..col + size).map(|c| (row, c)).collect()
        };

        // проверяем можно ли разметить корабль
        for &(r, c) in &coords {
            if r >= self.height || c >= self.width {
                return false;
            }
            if self.cells[r][c].is_some() {
                return false;
            }
        }

        // проверяем соседние клетки
        for &(r, c) in &coords {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < self.height as isize && nc >= 0 && nc < self.width as isize {
                        if self.cells[nr as usize][nc as usize].is_some() {
                            return false;
                        }
                    }
                }
            }
        }

        //размещаем корабль
        for &(r, c) in &coords {
            self.cells[r][c] = Some(Ship::new(coords.clone(), size));
        }

        true
    }

    pub fn place_ship(&mut self, ship: Ship) -> Result<(), String> {
        let coords = ship.coords;
        let size = ship.size;
        if coords.len() != size {
            return Err("Количество координат не соответствует размеру корабля".to_string());
        }

        for &(row, col) in &coords {
            if row >= self.height || col >= self.width {
                return Err("Координаты корабля выходят за границы поля".to_string());
            }
            if self.cells[row][col].is_some() {
                return Err("Корабль пересекается с другим кораблем".to_string());
            }
        }

        for &(row, col) in &coords {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;
                    if self.cells[nr as usize][nc as usize].is_some() {
                        return Err("Корабль касается другого корабля".to_string());
                    }
                }
            }
        }

        for &(row, col) in &coords {
            self.cells[row][col] = Some(Ship::new(coords.clone(), size));
        }

        Ok(())
    }


    pub fn shoot(&mut self, row: usize, col: usize) -> ShootResult {
        if row >= self.height || col >= self.width {
            panic!("Выстрел за пределы поля!");
        }

        if let Some(ship) = &mut self.cells[row][col] {
            let index = ship.coords.iter().position(|&coord| coord == (row, col)).unwrap();
            ship.hit(index);
            if ship.is_destroyed() {
                ShootResult::Destroy
            } else {
                ShootResult::Hit
            }
        } else {
            ShootResult::Miss
        }
    }

    pub fn all_ships_destroyed(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Some(ship) = cell {
                    if !ship.is_destroyed() {
                        return false;
                    }
                }
            }
        }
        return true
    }
}

pub fn place_ships_manually(board: &mut Board) -> Result<(), String> {
    let chips_to_place = vec![
        (4, 1),
        (3, 2),
        (2, 3),
        (1, 4),
    ];

    for &(size, count) in &chips_to_place {
        println!("Разметсите {}-палубный корабль. Введите координаты через пробел (например A1 A2 A3):", size);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<(usize, usize)> = input
            .trim()
            .split_whitespace()
            .map(|coord| parse_coordinates(coord).unwrap())
            .collect();
        
        if coords.len() != size {
            return Err("Неверное количество координат для корабля".to_string());
        }

        if let Err(err) = board.place_ship(coords, size) {
            return Err(err);
        }
    }
    Ok(())
}

pub fn parse_coordinates(input: &str) -> Option<(usize, uszie)> {
    let chars: Vec<char> = input.chars().collect();
    if chars.len() != 2 {
        return None;
    }
    
}
