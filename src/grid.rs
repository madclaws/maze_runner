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
            grid: Grid::prepare_grid(rows, cols)
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
                if row - 1 > -1 {
                    self.grid[index].north = Some(self.get_index(row - 1, col));
                }
                if row + 1 < self.rows {
                    self.grid[index].south = Some(self.get_index(row + 1, col));
                }
                if col - 1 > -1 {
                    self.grid[index].west = Some(self.get_index(row, col - 1));
                }
                if col + 1 < self.cols {
                    self.grid[index].east = Some(self.get_index(row, col + 1));
                }
            }
        }
    }

    pub fn each_cell(&self) -> IterHelper {
        self.into_iter()
    }

    pub fn each_cell_mut(&mut self) -> IterMutHelper {
        self.into_iter()
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

/* 
 self consuming struct iterator (for later reference)
 Here we need some field in struct to track the index
*/
// impl Iterator for Grid {
//     type Item = Cell;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.iter_index > (self.cols * self.rows) - 1 {
//             None
//         } else {
//             let cell = self.grid[self.iter_index as usize].clone();
//             self.iter_index += 1;
//             Some(cell)
//         }
//     }
// }

/* 
We have to implement an iterator on the grid vector (so that we can iterate through
each cells without using loops). For that we need to implement intoIterator, 
in its 3 forms, consuming iterator, non-consuming iterator and mutable non-consuming iterator
*/

// Consuming Iterator
/// Intermediate structure upon which we implement the IntoIterator
pub struct IntoIteratorHelper{
    iter: ::std::vec::IntoIter<Cell>
}

impl IntoIterator for Grid {
    type Item = Cell;
    type IntoIter = IntoIteratorHelper;

    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorHelper{
            iter: self.grid.into_iter()
        }
    }
}

impl Iterator for IntoIteratorHelper {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// Non consuming iterator
#[derive(Debug)]
pub struct IterHelper<'a> {
    iter: ::std::slice::Iter<'a, Cell>
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = IterHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper{
            iter: self.grid.iter()
        }
    }
}

impl<'a> Iterator for IterHelper<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// mutable non-consuming

pub struct IterMutHelper<'a> {
    iter: ::std::slice::IterMut<'a, Cell>
}

impl<'a> IntoIterator for &'a mut  Grid {
    type Item = &'a mut  Cell;
    type IntoIter = IterMutHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMutHelper{
            iter: self.grid.iter_mut()
        }
    }
}

impl<'a> Iterator for IterMutHelper<'a> {
    type Item = &'a mut Cell;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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

    #[test]
    fn test_grid_iterator_count_adapter() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        assert_eq!(grid.each_cell().count(), 16) 
    }

    #[test]
    fn test_grid_iterator_filter_adapter() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        // grid.each_cell().take(0..=4)
        assert_eq!(8, grid.each_cell().filter(|cell| cell.id % 2 == 0).count()) 
    }
}
