mod cell;
mod grid;

use cell::*;
use grid::*;

fn main() {
    println!("Mazes mazes everywhere..");
    
    println!("Creating a new cell at 0, 0 \n{:?}", Cell::new(0, 0, 0));
    println!("Create a grid of 4X4\n{:?}", Grid::new(4, 4));
}
