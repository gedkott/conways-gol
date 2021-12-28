use std::cell::{Ref, RefCell};
use std::fmt::Write;
use std::rc::Rc;

#[derive(Debug, std::cmp::PartialEq, Clone)]
pub enum State {
    Dead,
    Alive,
}

pub type GOLCellRef = Rc<RefCell<GOLCell>>;

#[derive(Debug)]
pub struct GOLCell {
    pub state: State,
    pub neighbors: Vec<GOLCellRef>,
}

impl std::fmt::Display for GOLCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}

impl GOLCell {
    pub fn new(state: State) -> Self {
        GOLCell {
            state,
            neighbors: vec![],
        }
    }

    pub fn add_neighbor(&mut self, cell: GOLCellRef) {
        self.neighbors.push(cell)
    }
}

pub struct Grid {
    pub cells: Vec<Vec<GOLCellRef>>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::Alive => '@',
            Self::Dead => '.',
        };
        f.write_char(char)
    }
}

impl Grid {
    pub fn with_cells(pre_made_cells: Vec<Vec<GOLCellRef>>) -> Self {
        Grid {
            cells: pre_made_cells,
        }
    }

    pub fn new() -> Self {
        Grid { cells: vec![] }
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Ref<'_, Vec<GOLCellRef>> {
        let cell = self.cells.get(row).and_then(|row| row.get(col));
        match cell {
            // See: https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap
            Some(c) => Ref::map(c.borrow(), |c| &c.neighbors),
            None => unreachable!(),
        }
    }

    // O(2 * n^2) - once for calculating the new states and once for actually updating the cells with their new states
    pub fn step(&mut self) {
        let mut new_states = vec![];

        // Second, iterate through self and update corresponding element in new_grid
        for i in 0..self.cells.len() {
            let row = self.cells.get(i);
            new_states.push(vec![]);
            for j in 0..row.map(|r| r.len()).unwrap_or(0) {
                let alive_neighbors = {
                    let neighbors = self.get_neighbors(i, j);
                    neighbors.iter().fold(0, |acc, cell| {
                        let add = if cell.borrow().state == State::Alive {
                            1
                        } else {
                            0
                        };
                        acc + add
                    })
                };
                let cell = self.cells.get_mut(i).and_then(|row| row.get_mut(j));
                // Should never see a non-Some type of cell so not perfect, but reasonable way to do this
                if let Some(c) = cell {
                    let borrowed_cell = c.borrow_mut();
                    let new_state = match (&borrowed_cell.state, alive_neighbors) {
                        (State::Alive, 2) => State::Alive,
                        (State::Alive, 3) => State::Alive,
                        (State::Dead, 3) => {
                            // become alive
                            State::Alive
                        }
                        _ => {
                            // stay dead or become dead
                            State::Dead
                        }
                    };
                    new_states[i].push(new_state);
                }
            }
        }

        // iterate through new_states updating each cell
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                // TODO(): do not like the need to clone the value of the state enum out of new_states
                self.cells[i][j].borrow_mut().state = new_states[i][j].clone();
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
