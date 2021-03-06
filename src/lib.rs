use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter, Result, Write};
use std::rc::Rc;

#[derive(Debug, std::cmp::PartialEq)]
pub enum State {
    Alive,
    Dead,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let char = match self {
            Self::Alive => '@',
            Self::Dead => '.',
        };
        f.write_char(char)
    }
}

pub type GOLCellRef = Rc<RefCell<GOLCell>>;

#[derive(Debug)]
pub struct GOLCell {
    pub state: State,
    pub neighbors: Vec<GOLCellRef>,
}

impl std::fmt::Display for GOLCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
        let cell: Option<&GOLCellRef> = self
            .cells
            .get(row)
            .and_then(|row: &Vec<GOLCellRef>| row.get(col));
        match cell {
            // See: https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap
            Some(c) => Ref::map(c.borrow(), |c| &c.neighbors),
            // technically this is reachable, but I am making the assumption that we know that the row and col provided are guaranteed to be in the grid
            None => unreachable!(),
        }
    }

    // O(2 * n) - once for calculating the new states and once for actually updating the cells with their new states
    // where n represents the number of cells in our grid
    pub fn step(&mut self) {
        let mut new_states = vec![];

        // First, iterate through self and figure out the new state for each i, j position in the grid, storing the new state in a an aux memory space
        for i in 0..self.cells.len() {
            let row: Option<&Vec<GOLCellRef>> = self.cells.get(i);
            new_states.push(vec![]);
            // unwrap_or is not as bad of an idea as unwrap
            for j in 0..row.map(|r: &Vec<GOLCellRef>| r.len()).unwrap_or(0) {
                let alive_neighbors = {
                    let neighbors: Ref<Vec<GOLCellRef>> = self.get_neighbors(i, j);
                    neighbors.iter().fold(0, |acc, cell: &GOLCellRef| {
                        let add = if cell.borrow().state == State::Alive {
                            1
                        } else {
                            0
                        };
                        acc + add
                    })
                };
                let cell: Option<&mut GOLCellRef> = self
                    .cells
                    .get_mut(i)
                    .and_then(|row: &mut Vec<GOLCellRef>| row.get_mut(j));
                // Should never see a non-Some type of cell so not perfect, but reasonable way to do this
                if let Some(c) = cell {
                    let borrowed_cell = c.borrow();
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

        // Second, iterate through new_states updating each corresponding position in self cells
        for (i, row) in new_states.iter_mut().enumerate() {
            for (j, state) in row.iter_mut().enumerate() {
                // TODO(gedkott): next line is able to panic because of the index operator even though it should not because we know the indexes are filled at this point
                std::mem::swap(state, &mut self.cells[i][j].borrow_mut().state);
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
