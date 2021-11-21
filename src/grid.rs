/* 
    Grid module, which implements Grid struct and its functionalities

    Grid is a wrapper of 2D array of Cells.
*/
use rand::Rng;
use crate::cell::*;

/// Grid representation
/// 
/// * `rows` - No:of rows of grid
/// * `cols` - No:of cols of grid
/// * `grid` - 2D matrix of cells

#[derive(Debug)]
pub struct Grid {
    rows: u32,
    cols: u32,
    grid: Vec<Cell>
}

impl Grid {
    pub fn new(rows: u32, cols: u32) -> Self {
        Grid{rows, cols, grid: Grid::prepare_grid(rows, cols)}
    }

    pub fn get_size(&self) -> u32 {
        self.rows * self.cols
    }

    fn get_random_cell(&self) -> Option<&Cell>{
        let rand_row = rand::thread_rng().gen_range(0..self.rows);
        let rand_col = rand::thread_rng().gen_range(0..self.rows);
        self.grid.get(self.get_index(rand_row, rand_col))
    }

    fn get_cell(&self, row: u32, col: u32) -> Option<&Cell>{
        self.grid.get(self.get_index(row, col))
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.cols) + col) as usize
    }


    /// Creates a 2D matrix of cells
    fn prepare_grid(rows: u32, cols: u32) -> Vec<Cell>{
        let mut grid: Vec<Cell> = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                grid.push(Cell::new((row * cols) + col, row, col))
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests{
    use super::Grid;
    
    #[test]
    fn creating_grid() {
        let grid = Grid::new(4, 4);
        if let Some(cell) = grid.get_cell(3, 3) {
            assert_eq!(cell.id, 15);
        }
    }

}
