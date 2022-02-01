/*
    Grid module, which implements Grid struct and its functionalities

    Grid is a wrapper of 2D array of Cells.
*/
use crate::cell::*;
use crate::distances::*;
use crate::mask::*;

use rand::Rng;
/// Grid representation
///
/// * `rows` - No:of rows of grid
/// * `cols` - No:of cols of grid
/// * `grid` - 2D matrix of cells

#[derive(Debug)]
pub struct Grid {
    pub rows: i32,
    pub cols: i32,
    pub grid: Vec<Cell>,
}

pub trait Algorithm {
    fn on<'a>(&'a self, grid: &'a mut Grid, start_cell: i32);
    fn get_name(&self) -> String;
}

impl Grid {
    pub fn new(rows: i32, cols: i32) -> Self {
        Grid {
            rows,
            cols,
            grid: Grid::prepare_grid(rows, cols),
        }
    }

    pub fn new_from_mask(mask: &Mask) -> Self {
        Grid {
            rows: mask.rows,
            cols: mask.cols,
            grid: Grid::prepare_grid_from_mask(mask),
        }
    }

    pub fn get_size(&self) -> u32 {
        (self.rows * self.cols) as u32
    }

    pub fn get_random_cell(&self) -> Option<&Cell> {
        let rand_row = rand::thread_rng().gen_range(0..self.rows);
        let rand_col = rand::thread_rng().gen_range(0..self.rows);
        self.grid.get(self.get_index(rand_row, rand_col) as usize)
    }

    fn _get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
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
                if !self.grid[index].active {
                    continue;
                }
                if row - 1 > -1 && self.grid[self.get_index(row - 1, col) as usize].active {
                    self.grid[index].north = Some(self.get_index(row - 1, col));
                }
                if row + 1 < self.rows && self.grid[self.get_index(row + 1, col) as usize].active {
                    self.grid[index].south = Some(self.get_index(row + 1, col));
                }
                if col - 1 > -1 && self.grid[self.get_index(row, col - 1) as usize].active {
                    self.grid[index].west = Some(self.get_index(row, col - 1));
                }
                if col + 1 < self.cols && self.grid[self.get_index(row, col + 1) as usize].active {
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

    // Doing biderectional links with cells
    pub fn link_cells(&mut self, cell_id: i32, to_link_cell_id: i32) {
        self.grid[cell_id as usize].link(to_link_cell_id);
        self.grid[to_link_cell_id as usize].link(cell_id);
    }

    // Doing biderectional unlinks with cells
    pub fn unlink_cells(&mut self, cell_id: i32, to_link_cell_id: i32) {
        self.grid[cell_id as usize].unlink(to_link_cell_id);
        self.grid[to_link_cell_id as usize].unlink(cell_id);
    }

    // Renders the maze
    pub fn render(&self) {
        let mut output = format!("{}{}{}", "+", "---+".repeat(self.cols as usize), "\n");
        for row in 0..self.rows {
            let mut top: String = String::from("|");
            let mut bottom: String = String::from("+");
            for col in 0..self.cols {
                let body = String::from("   ");
                let eastern_boundary;
                if let Some(id) = self.grid[((row * self.cols) + col) as usize].east {
                    if self.grid[((row * self.cols) + col) as usize].is_linked(id) {
                        eastern_boundary = " ".to_owned();
                    } else {
                        eastern_boundary = "|".to_owned();
                    }
                } else {
                    eastern_boundary = "|".to_owned();
                }
                top = format!("{}{}{}", top, body, eastern_boundary);

                let southern_boundary;
                if let Some(id) = self.grid[((row * self.cols) + col) as usize].south {
                    if self.grid[((row * self.cols) + col) as usize].is_linked(id) {
                        southern_boundary = "   ".to_owned();
                    } else {
                        southern_boundary = "---".to_owned();
                    }
                } else {
                    southern_boundary = "---".to_owned();
                }
                bottom = format!("{}{}{}", bottom, southern_boundary, "+");
            }
            output = format!("{}{}{}", output, top, "\n");
            output = format!("{}{}{}", output, bottom, "\n");
        }
        println!("{}", output);
    }

    pub fn distances(&self, root_cell_id: i32) -> Distances {
        let mut distances = Distances::new(root_cell_id);
        let mut frontier: Vec<i32> = vec![root_cell_id];
        println!("Starting calculating relative distance from the root");
        while !frontier.is_empty() {
            let mut next_frontiers: Vec<i32> = Vec::new();
            for cell_id in &frontier {
                let linked_cells = self.grid[*cell_id as usize].get_linked_cells();
                for linked_cell in &linked_cells {
                    if distances.get_distance(*linked_cell).is_none() {
                        distances.set_distance(
                            *linked_cell,
                            *distances.get_distance(*cell_id).unwrap() + 1,
                        );
                        next_frontiers.push(*linked_cell);
                    }
                }
            }
            frontier.clear();
            frontier = next_frontiers;
        }
        println!("Ending calculation of relative distance from the root\n\n");
        // println!("{:?}", distances);
        distances
    }

    pub fn breadcrumbs(
        &self,
        goal_cell_id: i32,
        root_cell_id: i32,
        distances: &Distances,
    ) -> Distances {
        let mut breadcrumbs = Distances::new(root_cell_id);
        let mut current_cell_id = goal_cell_id;
        breadcrumbs.set_distance(
            current_cell_id,
            *distances.get_distance(current_cell_id).unwrap(),
        );
        while current_cell_id != root_cell_id {
            let current_cell_distance = distances.get_distance(current_cell_id).unwrap();
            let linked_cells = self.grid[current_cell_id as usize].get_linked_cells();
            for linked_cell in &linked_cells {
                if distances.get_distance(*linked_cell).unwrap() < current_cell_distance {
                    current_cell_id = *linked_cell;
                    breadcrumbs.set_distance(
                        current_cell_id,
                        *distances.get_distance(current_cell_id).unwrap(),
                    );
                    break;
                }
            }
        }
        println!("BREADCRUMBS\n {:?}", breadcrumbs);
        breadcrumbs
    }
    pub fn get_deadend_cells(&self) -> Vec<i32> {
        let mut deadend_cells: Vec<i32> = Vec::new();
        for cell in &self.grid {
            if cell.get_linked_cells().len() == 1 {
                deadend_cells.push(cell.id);
            }
        }
        deadend_cells
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

    fn prepare_grid_from_mask(mask: &Mask) -> Vec<Cell> {
        let mut grid: Vec<Cell> = Vec::new();
        for row in 0..mask.rows {
            for col in 0..mask.cols {
                let mut cell = Cell::new((row * mask.cols) + col, row, col);
                if !mask.get((row, col)) {
                    cell.active = false;
                }
                grid.push(cell);
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
pub struct IntoIteratorHelper {
    iter: ::std::vec::IntoIter<Cell>,
}

impl IntoIterator for Grid {
    type Item = Cell;
    type IntoIter = IntoIteratorHelper;

    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorHelper {
            iter: self.grid.into_iter(),
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
    iter: ::std::slice::Iter<'a, Cell>,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = IterHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper {
            iter: self.grid.iter(),
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
    iter: ::std::slice::IterMut<'a, Cell>,
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut Cell;
    type IntoIter = IterMutHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMutHelper {
            iter: self.grid.iter_mut(),
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
        if let Some(cell) = grid._get_cell(3, 3) {
            assert_eq!(cell.id, 15);
        }
    }

    #[test]
    fn test_configure_cells() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        if let Some(cell) = grid._get_cell(3, 3) {
            assert_eq!(cell.east, None);
            assert_eq!(cell.south, None);
            assert_eq!(cell.west, Some(14));
            assert_eq!(cell.north, Some(11));
        }
        if let Some(cell) = grid._get_cell(2, 1) {
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

    #[test]
    fn test_link_and_unlink_cells() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        grid.link_cells(0, 1);
        let linked_cells_0 = grid.grid[0].get_linked_cells();
        assert_eq!(linked_cells_0.into_iter().find(|x| *x == 1), Some(1));
        let linked_cells_1 = grid.grid[1].get_linked_cells();
        assert_eq!(linked_cells_1.into_iter().find(|x| *x == 0), Some(0));
        grid.unlink_cells(0, 1);
        let linked_cells_0 = grid.grid[0].get_linked_cells();
        assert_eq!(linked_cells_0.into_iter().find(|x| *x == 1), None);
        let linked_cells_1 = grid.grid[1].get_linked_cells();
        assert_eq!(linked_cells_1.into_iter().find(|x| *x == 0), None);
    }

    #[test]
    fn test_distances() {
        let mut grid = Grid::new(4, 4);
        grid.configure_cells();
        grid.link_cells(0, 1);
        let distances = grid.distances(0);
        assert_eq!(distances.get_distance(1), Some(&1));
        assert_eq!(distances.get_distance(4), None);
    }
}
