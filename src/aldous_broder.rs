/*
    Implementation of Aldous-broder algorithm

    This is an unbiased algorithm. The general concept is we move randomly through
    the grid, until all cells are linked.
*/

use rand::Rng;

use crate::grid::*;

pub fn on(grid: &mut Grid) -> &mut Grid {
    let mut current_cell_id: i32 = rand::thread_rng().gen_range(0..grid.grid.len()) as i32;
    let mut unvisited_cells = grid.grid.len() - 1;

    while unvisited_cells > 0 {
        let neightbours = grid.grid[current_cell_id as usize].get_neighbours();
        let random_neighbour = neightbours[rand::thread_rng().gen_range(0..neightbours.len())];
        if grid.grid[random_neighbour as usize]
            .get_linked_cells()
            .is_empty()
        {
            grid.link_cells(current_cell_id, random_neighbour);
            unvisited_cells -= 1;
        }
        current_cell_id = random_neighbour;
    }
    grid
}
