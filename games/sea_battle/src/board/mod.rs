use crate::ship::{Ship, ShootResult};

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

    pub fn place_ship(&mut self, ship: &Ship) -> Result<(), String> {
        for &(row, col) in &ship.coords {
            if row >= self.height || col >= self.width {
                return Err("Координаты корабля выходят за пределы поля".to_string());
            }
            if self.cells[row][col].is_some() {
                return Err("Корабль пересекается с другим кораблем".to_string());
            }
        }

        for &(row, col) in &ship.coords {
            self.cells[row][col] = Some(ship.clone())
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
