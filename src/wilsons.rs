/*
    Implementing Wilson's algorithm

    An unbiased random-walk maze, where slow start and fast end
*/

use rand::Rng;

use crate::grid::*;

pub struct Wilsons {}

impl Algorithm for Wilsons {
    fn on(&self, grid: &mut Grid, _start_cell: i32) {
        let mut unvisited_cells: Vec<i32> = (0..(grid.cols * grid.rows)).collect();
        unvisited_cells.remove(rand::thread_rng().gen_range(0..unvisited_cells.len()));

        while !unvisited_cells.is_empty() {
            let mut cell = unvisited_cells[rand::thread_rng().gen_range(0..unvisited_cells.len())];
            let mut path: Vec<i32> = vec![cell];

            while unvisited_cells.contains(&cell) {
                let neightbours = grid.grid[cell as usize].get_neighbours();
                cell = neightbours[rand::thread_rng().gen_range(0..neightbours.len())];
                if let Some(position) = path.iter().position(|c| *c == cell) {
                    path.truncate(position + 1);
                } else {
                    path.push(cell);
                }
            }

            for index in 0..=path.len() - 2 {
                grid.link_cells(path[index], path[index + 1]);
                let path_cell_position = unvisited_cells
                    .iter()
                    .position(|cell| *cell == path[index])
                    .unwrap();
                unvisited_cells.remove(path_cell_position);
            }
        }
    }

    fn get_name(&self) -> String {
        "Wilsons".to_owned()
    }
}
