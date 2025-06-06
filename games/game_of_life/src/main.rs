use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if *self == Cell::Alive { "■" } else { " " })
    }
}

struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width * height];
        Universe {
            width,
            height,
            cells,
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, state: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx] = state;
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {

    }

    fn tick(&mut self) {

    }

    fn render(&self) -> String {

    }
}

fn main() {
    println!("Hello, world!");
}
