mod cell;
mod grid;

use cell::*;
use grid::*;

fn main() {
    println!("Mazes mazes everywhere..");

    println!("Creating a new cell at 0, 0 \n{:?}", Cell::new(0, 0, 0));
    let mut grid = Grid::new(4, 4);
    grid.configure_cells();
    // grid.prepare_grid();
    println!("Create a grid of 4X4\n{:?}", grid);
}
