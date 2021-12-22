use conways_gol::{Cell, Grid, State};

fn main() {
    /*
       [
         [1, 0, 1, 0, 1]
         [1, 0, 1, 0, 1]
         [1, 0, 1, 0, 1]
         [1, 0, 1, 0, 1]
       ]
    */
    let mut columns = vec![];
    let width = 5;
    let height = 4;
    for _ in 0..height {
        let mut row = vec![];
        for i in 0..width {
            let cell = Cell {
                state: if i % 2 == 0 {
                    State::Alive
                } else {
                    State::Dead
                },
            };
            row.push(cell);
        }
        columns.push(row);
    }

    let mut grid = Grid::with_cells(columns);

    for i in 0..height {
        for j in 0..width {
            print!(" {} ", grid.cells.get(i).and_then(|row| row.get(j)).unwrap());
        }
        println!();
    }

    for _ in 0..10 {
        println!();
        grid.step();

        for i in 0..height {
            for j in 0..width {
                print!(" {} ", grid.cells.get(i).and_then(|row| row.get(j)).unwrap());
            }
            println!();
        }
    }
}
