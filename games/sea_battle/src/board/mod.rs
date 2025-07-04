use crate::ship::{Ship, ShootResult};
use rand::Rng;
use std::{io, rc::Rc, cell::RefCell};


pub struct Board {
    pub cells: Vec<Vec<Option<Rc<RefCell<Ship>>>>>,
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
        let ship = Rc::new(RefCell::new(Ship::new(coords.clone(), size)));
        for &(r, c) in &coords {
            self.cells[r][c] = Some(Rc::clone(&ship));
        }

        true
    }

    pub fn place_ship(&mut self, ship: Rc<RefCell<Ship>>) -> Result<(), String> {
        let ship_ref = ship.borrow_mut();
        let coords = &ship_ref.coords;
        let size = ship_ref.size;
        if coords.len() != size {
            return Err("Количество координат не соответствует размеру корабля".to_string());
        }

        for &(row, col) in coords {
            if row >= self.height || col >= self.width {
                return Err("Координаты корабля выходят за границы поля".to_string());
            }
            if self.cells[row][col].is_some() {
                return Err("Корабль пересекается с другим кораблем".to_string());
            }
        }

        for &(row, col) in coords {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;
                    if nr >= 0 && nr < self.height as isize && nc >=0 && nc < self.width as isize {
                        if self.cells[nr as usize][nc as usize].is_some() {
                            return Err("Корабль касается другого корабля".to_string());
                        }
                    }
                }
            }
        }

        for &(row, col) in coords {
            self.cells[row][col] = Some(Rc::clone(&ship));
        }
        Ok(())
    }

    pub fn shoot(&mut self, row: usize, col: usize) -> ShootResult {
        if row >= self.height || col >= self.width {
            panic!("Выстрел за пределы поля!");
        }

        if let Some(ship) = &mut self.cells[row][col] {
            let mut ship_ref = ship.borrow_mut();
            if let Some(index) = ship_ref.coords.iter().position(|&coord| coord == (row, col)) {
                ship_ref.hit(index);
                if ship_ref.is_destroyed() {
                    return ShootResult::Destroy;
                } else {
                    return ShootResult::Hit;
                }
            } else {
                panic!("Несовпадение координат корабля");
            }
        }
        ShootResult::Miss
    }

    pub fn all_ships_destroyed(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Some(ship) = cell {
                    let ship_ref = ship.borrow();
                    if !ship_ref.is_destroyed() {
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
        for _ in 0..count {
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
    
            let ship = Rc::new(RefCell::new(Ship::new(coords.clone(), size)));
    
            if let Err(err) = board.place_ship(ship) {
                return Err(err);
            }
        }
    }
    Ok(())
}

pub fn parse_coordinates(input: &str) -> Option<(usize, usize)> {
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

pub trait ShipPlacer {
    fn place_ships(&self, board: &mut Board) -> Result<(), String>;
}

pub struct AutoShipPlacer;
impl ShipPlacer for AutoShipPlacer {
    fn place_ships(&self, board: &mut Board) -> Result<(), String> {
        if board.place_ships_randomly().is_err() {
            println!("Не удалось разместить корабли автоматически. Перегенерация...");
            loop {
                if board.place_ships_randomly().is_ok() {
                    break;
                }
            }
        }
        Ok(())
    }
}

pub struct ManualShipPlacer;
impl ShipPlacer for ManualShipPlacer {
    fn place_ships(&self, board: &mut Board) -> Result<(), String> {
        if place_ships_manually(board).is_err() {
            panic!("Не удалось разместить корабли вручную");
        }
        Ok(())
    }
}
