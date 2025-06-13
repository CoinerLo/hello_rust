use crate::ship::Ship;

#[derive(Debug, PartialEq)]
pub enum ShootResult {
    Miss,
    Hit,
    Destroy,
}

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
        
    }
}
