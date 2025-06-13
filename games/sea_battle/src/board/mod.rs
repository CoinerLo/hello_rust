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

    pub fn place_ship(&mut self, ship: &Ship) -> Resuil<(), string> {

    }

    pub fn shoot(&mut self, row: usize, col: usize) -> ShootResult {

    }
}
