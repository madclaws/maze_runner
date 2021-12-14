/*
    Grid module, which implements Grid struct and its functionalities

    Grid is a wrapper of 2D array of Cells.
*/
use crate::cell::*;
use rand::Rng;

/// Grid representation
///
/// * `rows` - No:of rows of grid
/// * `cols` - No:of cols of grid
/// * `grid` - 2D matrix of cells

#[derive(Debug)]
pub struct Grid {
    rows: i32,
    cols: i32,
    grid: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: i32, cols: i32) -> Self {
        Grid {
            rows,
            cols,
            grid: Grid::prepare_grid(rows, cols),
        }
    }

    pub fn get_size(&self) -> u32 {
        (self.rows * self.cols) as u32
    }

    fn get_random_cell(&self) -> Option<&Cell> {
        let rand_row = rand::thread_rng().gen_range(0..self.rows);
        let rand_col = rand::thread_rng().gen_range(0..self.rows);
        self.grid.get(self.get_index(rand_row, rand_col) as usize)
    }

    fn get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
        self.grid.get(self.get_index(row, col) as usize)
    }

    fn get_index(&self, row: i32, col: i32) -> i32 {
        ((row * self.cols) + col) as i32
    }

    /// Update the neighbours of each cell
    pub fn configure_cells(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = self.get_index(row, col) as usize;
                if row - 1 > 0 {
                    self.grid[index].north = Some(self.get_index(row - 1, col));
                }
                if row + 1 < self.rows {
                    self.grid[index].south = Some(self.get_index(row + 1, col));
                }
                if col - 1 > 0 {
                    self.grid[index].west = Some(self.get_index(row, col - 1));
                }
                if col + 1 < self.cols {
                    self.grid[index].east = Some(self.get_index(row, col + 1));
                }
            }
        }
    }

    /// Creates a 2D matrix of cells
    fn prepare_grid(rows: i32, cols: i32) -> Vec<Cell> {
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
mod tests {
    use super::Grid;

    #[test]
    fn creating_grid() {
        let grid = Grid::new(4, 4);
        if let Some(cell) = grid.get_cell(3, 3) {
            assert_eq!(cell.id, 15);
        }
    }

    #[test]
    fn test_configure_cells() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        if let Some(cell) = grid.get_cell(3, 3) {
            assert_eq!(cell.east, None);
            assert_eq!(cell.south, None);
            assert_eq!(cell.west, Some(14));
            assert_eq!(cell.north, Some(11));
        }
        if let Some(cell) = grid.get_cell(2, 1) {
            assert_eq!(cell.east, Some(10));
            assert_eq!(cell.south, Some(13));
            assert_eq!(cell.west, Some(8));
            assert_eq!(cell.north, Some(5));
        }
    }
}
