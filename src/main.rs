use std::{cell::RefCell, rc::Rc};

use conways_gol::{GOLCell, Grid, State};

fn populate_neighbors_from_grid_for_cell_at(grid: &Grid, row: usize, col: usize) {
    // TODO(gedkott): next line is able to panic even though it should not
    let mut mut_brwed_cell = grid.cells[row][col].borrow_mut();
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
        let neighbor = grid.cells.get(i).and_then(|r| r.get(j));
        if let Some(c) = neighbor {
            mut_brwed_cell.add_neighbor(c.clone());
            println!(
                "cell at {:?} is getting neighbor at {:?}",
                (row, col),
                (i, j)
            )
        }
    }

    println!(
        "cell at {:?} is getting {:?} neighbors",
        (row, col),
        mut_brwed_cell.neighbors.len()
    )
}

fn main() {
    // construct blinker oscillator
    let height = 5;
    let width = 5;

    let mut pre_made_cells = vec![];

    for _i in 0..height {
        let mut row = vec![];
        for _j in 0..width {
            let new_cell = Rc::new(RefCell::new(GOLCell::new(State::Dead)));
            row.push(new_cell);
        }
        pre_made_cells.push(row);
    }

    let mut grid = Grid::with_cells(pre_made_cells);

    for i in 0..height {
        for j in 0..width {
            populate_neighbors_from_grid_for_cell_at(&grid, i, j);
        }
    }

    // update only cells I want to start alive while rest default to dead
    let alive_row = 2;
    let alive_cols = [1, 2, 3];

    for col in alive_cols {
        grid.cells[alive_row][col].borrow_mut().state = State::Alive;
    }

    println!("Initial State");
    println!();
    for i in 0..height {
        for j in 0..width {
            print!(
                " {} ",
                grid.cells
                    .get(i)
                    .and_then(|row| row.get(j))
                    .unwrap()
                    .borrow()
            );
        }
        println!();
    }

    for _ in 0..10 {
        println!();
        grid.step();

        for i in 0..height {
            for j in 0..width {
                print!(
                    " {} ",
                    grid.cells
                        .get(i)
                        .and_then(|row| row.get(j))
                        .unwrap()
                        .borrow()
                );
            }
            println!();
        }
    }
}
