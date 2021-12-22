use std::fmt::Write;

#[derive(Debug, std::cmp::PartialEq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Debug)]
pub struct Cell {
    pub state: State,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            &Self::Alive => '@',
            &Self::Dead => '.',
        };
        f.write_char(char)
    }
}

impl Grid {
    pub fn with_cells(pre_made_cells: Vec<Vec<Cell>>) -> Self {
        Grid {
            cells: pre_made_cells,
        }
    }

    pub fn new() -> Self {
        Grid { cells: vec![] }
    }

    // wrapping sub is used instead of doing unsigned math correctly
    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<&Cell> {
        let mut neighbors = vec![];
        for (i, j) in [
            (row.wrapping_sub(1), col.wrapping_sub(1)), // top left
            (row, col.wrapping_sub(1)),                 // top
            (row + 1, col.wrapping_sub(1)),             // top right
            (row.wrapping_sub(1), col),                 // mid left
            (row + 1, col),                             // mid right
            (row.wrapping_sub(1), col + 1),             // bottom left
            (row, col + 1),                             // bottom
            (row + 1, col + 1),                         // bottom right
        ] {
            let cell = self.cells.get(i).and_then(|row| row.get(j));
            match cell {
                Some(c) => neighbors.push(c),
                None => (),
            }
        }
        neighbors
    }

    pub fn step(&mut self) {
        for i in 0..self.cells.len() {
            let row = self.cells.get(i);
            for j in 0..row.map(|r| r.len()).unwrap_or(0) {
                let neighbors = self.get_neighbors(i, j);
                // println!("{:?}", neighbors);
                let alive_neighbors = neighbors.iter().fold(0, |acc, cell| {
                    let add = if cell.state == State::Alive { 1 } else { 0 };
                    acc + add
                });
                let cell = self.cells.get_mut(i).and_then(|row| row.get_mut(j));
                match cell {
                    Some(c) => {
                        match (&c.state, alive_neighbors) {
                            (State::Alive, 2) => {
                                // stay alive; no-op
                            }
                            (State::Alive, 3) => {
                                // stay alive; no-op
                            }
                            (State::Dead, 3) => {
                                // become alive
                                c.state = State::Alive;
                            }
                            _ => {
                                // stay dead or become dead
                                c.state = State::Dead;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
