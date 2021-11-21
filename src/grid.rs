/* 
    Grid module, which implements Grid struct and its functionalities

    Grid is a wrapper of 2D array of Cells.
*/

use crate::cell::*;

/// Grid representation
/// 
/// * `rows` - No:of rows of grid
/// * `cols` - No:of cols of grid
/// * `grid_matrix` - 2D matrix of cells

#[derive(Debug)]
pub struct Grid {
    rows: u32,
    cols: u32,
    grid_matrix: Vec<Cell> 
}

impl Grid {
    /// Returns a new Grid
    /// 
    /// # Arguments
    /// * `rows` - No:of rows grid have
    /// * `cols` - No:of cols grid have
    pub fn new(rows: u32, cols: u32) -> Self {
        Grid{rows: rows, cols: cols, grid_matrix: Vec::new()}
    }
}
