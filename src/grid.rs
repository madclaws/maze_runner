/* 
    Grid module, which implements Grid struct and its functionalities

    Grid is a wrapper of 2D array of Cells.
*/

use crate::cell::*;

/// Grid representation
/// 
/// * `rows` - No:of rows of grid
/// * `cols` - No:of cols of grid

#[derive(Debug)]
pub struct Grid {
    rows: u32,
    cols: u32,
    grid: Vec<Cell> 
}

impl Grid {
    /// Returns a new Grid
    /// 
    /// # Arguments
    /// * `rows` - No:of rows grid have
    /// * `cols` - No:of cols grid have
    pub fn new(rows: u32, cols: u32) -> Self {
        Grid{rows: rows, cols: cols, grid: Vec::new()}
    }
}
