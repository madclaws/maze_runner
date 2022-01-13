mod cell;
mod grid;
mod binary_tree;
mod sidewinder;
// use cell::*;
use grid::*;
// use binary_tree::*;
fn main() {
    println!("Mazes mazes everywhere..");

    let mut grid = Grid::new(10, 10);
    grid.configure_cells();

    // binary_tree::on(&mut grid);
    // grid.render();

    sidewinder::on(&mut grid);
    grid.render();
}
