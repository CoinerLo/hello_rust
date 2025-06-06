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
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += match self.cells[idx] {
                    Cell::Alive => 1,
                    Cell::Dead => 0,
                }

            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwisw, _) => otherwisw,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn render(&self) -> String {
        self.cells
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|cell| format!("{}", cell))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let mut universe = Universe::new(10, 5);

    universe.set_cell(2, 3, Cell::Alive);
    universe.set_cell(2, 4, Cell::Alive);
    universe.set_cell(2, 5, Cell::Alive);
    universe.set_cell(3, 3, Cell::Alive);
    universe.set_cell(3, 4, Cell::Alive);
    universe.set_cell(3, 5, Cell::Alive);
    universe.set_cell(4, 4, Cell::Alive);
    universe.set_cell(1, 3, Cell::Alive);
    universe.set_cell(1, 5, Cell::Alive);

    for state in 0..5 {
        println!("{} -----------------------", state);
        println!("{}", universe.render());
        universe.tick();
    }
}
