#[derive(Debug, PartialEq)]
pub enum ShootResult {
    Miss,
    Hit,
    Destroy,
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub coords: Vec<(usize, usize)>,
    pub size: usize,
    pub hits: Vec<bool>,
}

impl Ship {
    pub fn new(coords: Vec<(usize, usize)>, size: usize) -> Self {
        Ship {
            coords,
            size,
            hits: vec!(false; size),
        }
    }

    pub fn hit(&mut self, index: usize) {
        if index < self.hits.len() {
            self.hits[index] = true;
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.hits.iter().all(|&hit| hit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use std::{rc::Rc, cell::RefCell};

    #[test]
    fn test_ship_destroyed() {
        let mut board = Board::new(10, 10);
        let ship = Rc::new(RefCell::new(Ship::new(vec![(0, 0), (0, 1)], 2)));

        board.place_ship(ship).unwrap();
        assert_eq!(board.shoot(0, 0), ShootResult::Hit);
        assert_eq!(board.shoot(0, 2), ShootResult::Miss);
        assert_eq!(board.shoot(0, 1), ShootResult::Destroy);
    }
}
