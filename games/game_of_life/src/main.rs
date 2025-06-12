use std::{io::stdout, io::Result, time::Duration, thread};
use crossterm::{
    cursor,
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
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
        for delta_row in [-1_isize, 0, 1].iter().cloned() {
            for delta_col in [-1_isize, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = row as isize + delta_row;
                let neighbor_col = col as isize + delta_col;
                if neighbor_row >= 0 && neighbor_row < self.height as isize && neighbor_col >= 0 && neighbor_col < self.width as isize {
                    let idx = self.get_index(neighbor_row as usize, neighbor_col as usize);
                    count += match self.cells[idx] {
                        Cell::Alive => 1,
                        Cell::Dead => 0,
                    }
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
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn render(&self, selected_row: Option<usize>, selected_col: Option<usize>) -> String {
        self.cells
            .chunks(self.width)
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        if Some(y) == selected_row && Some(x) == selected_col {
                            format!("*")
                        } else if *cell == Cell::Alive {
                            "■".to_string()
                        } else {
                            " ".to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn count_live_cells(&self) -> usize {
        self.cells.iter().filter(|&&cell| cell == Cell::Alive).count()
    }
}

fn main() -> Result<()> {
    let mut universe = Universe::new(20, 10);

    universe.set_cell(2, 3, Cell::Alive);
    universe.set_cell(2, 4, Cell::Alive);
    universe.set_cell(2, 5, Cell::Alive);
    universe.set_cell(3, 3, Cell::Alive);
    universe.set_cell(3, 4, Cell::Alive);
    universe.set_cell(3, 5, Cell::Alive);
    universe.set_cell(4, 4, Cell::Alive);
    universe.set_cell(1, 3, Cell::Alive);
    universe.set_cell(1, 5, Cell::Alive);

    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;

    let mut is_paused = false;
    let mut tick_duration = Duration::from_millis(1000);
    let max_ticks = 100;
    let mut ticks = 0;
    let mut selected_row: usize = 0;
    let mut selected_col: usize = 0;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') => is_paused = !is_paused,
                    KeyCode::Char('+') => tick_duration = tick_duration.saturating_sub(Duration::from_millis(100)),
                    KeyCode::Char('-') => tick_duration += Duration::from_millis(100),
                    KeyCode::Up => selected_row = selected_row.saturating_sub(1),
                    KeyCode::Down => selected_row = (selected_row + 1).min(universe.height - 1),
                    KeyCode::Left => selected_col = selected_col.saturating_sub(1),
                    KeyCode::Right => selected_col = (selected_col + 1).min(universe.width - 1),
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if is_paused {
                            let current_state = universe.cells[universe.get_index(selected_row, selected_col)];
                            let new_state = if current_state == Cell::Alive {
                                Cell::Dead
                            } else {
                                Cell::Alive
                            };
                            universe.set_cell(selected_row, selected_col, new_state);
                        }
                    },
                    _ => {}
                }
            }
        }

        if !is_paused {
            universe.tick();
        }

        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        let rendered = if is_paused {
            universe.render(Some(selected_row), Some(selected_col))
        } else {
            universe.render(None, None)
        };

        for (y, line) in rendered.lines().enumerate() {
            execute!(stdout, cursor::MoveTo(0, y as u16))?;
            for ch in line.chars() {
                if ch == '*' {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        SetBackgroundColor(Color::Black),
                        Print(ch),
                        ResetColor,
                    )?;
                } else if ch == '■' {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Green),
                        SetBackgroundColor(Color::Black),
                        Print(ch),
                        ResetColor,
                    )?;
                } else {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::Black),
                        Print(ch),
                        ResetColor,
                    )?;
                }
            }
        }

        thread::sleep(tick_duration);

        ticks += 1;

        if ticks >= max_ticks {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbor_count() {
        let mut universe = Universe::new(3, 3);
        universe.set_cell(0, 1, Cell::Alive);
        universe.set_cell(1, 2, Cell::Alive);
        universe.set_cell(2, 0, Cell::Alive);

        assert_eq!(universe.live_neighbor_count(1, 1), 3);
        assert_eq!(universe.live_neighbor_count(2, 0), 0);
        assert_eq!(universe.live_neighbor_count(0, 0), 1);
        assert_eq!(universe.live_neighbor_count(2, 2), 1);
    }

    #[test]
    fn test_tick_rules() {
        let mut universe = Universe::new(3, 3);

        universe.set_cell(0, 1, Cell::Alive);
        universe.set_cell(1, 2, Cell::Alive);
        universe.set_cell(2, 0, Cell::Alive);

        universe.tick();
        assert_eq!(universe.cells[universe.get_index(1, 1)], Cell::Alive);

        let mut universe = Universe::new(3, 3);

        universe.set_cell(0, 1, Cell::Alive);
        universe.set_cell(1, 2, Cell::Alive);
        universe.set_cell(2, 0, Cell::Alive);

        universe.tick();
        assert_eq!(universe.cells[universe.get_index(0, 1)], Cell::Dead);

        let mut universe = Universe::new(3, 3);
        universe.set_cell(1, 1, Cell::Alive);
        universe.tick();
        assert_eq!(universe.cells[universe.get_index(1, 1)], Cell::Dead);
    }
}
