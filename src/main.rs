mod cell;
mod grid;

use cell::*;
use grid::*;

fn main() {
    println!("Mazes mazes everywhere..");

    // println!("Creating a new cell at 0, 0 \n{:?}", Cell::new(0, 0, 0));
    let mut grid = Grid::new(4, 4);
    grid.configure_cells();
    // for cell in grid {
    //     println!("Cell => #{:?}", cell);
    // }
    // println!("10th Cell => {:?}", grid.each_cell().take(5));
    println!("10th Cell => {:?}", grid.each_cell().filter(|cell| cell.id >= 4 && cell.id < 9).collect::<Vec<_>>());

    // println!("10th Cell => {:?}", grid.take(2));
    
    // grid.prepare_grid();
    // println!("Create a grid of 4X4\n{:?}", grid);
}
