#[derive(Debug, PartialEq)]
pub enum ShootResult {
    Miss,
    Hit,
    Destroy,
}

#[derive(Debug)]
pub struct Ship {
    pub coords: Vec<(usize, usize)>,
    pub hits: Vec<bool>,
    pub size: usize,
}

impl Ship {
    pub fn new(coords: Vec<(usize, usize)>, size: usize) -> Self {
        Ship {
            coords,
            hits: vec!(false; size),
            size,
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
