/*
    Implementing Hunt and kill algorithm
    An biased random walk
*/

use crate::grid::*;
use rand::Rng;

pub struct HuntAndKill;

impl Algorithm for HuntAndKill {
    fn on(&self, grid: &mut Grid) {
        let mut current_cell_id: Option<i32> =
            Some(rand::thread_rng().gen_range(0..grid.grid.len()) as i32);

        while current_cell_id.is_some() {
            let unvisited_neightbours: Vec<i32> = grid.grid[current_cell_id.unwrap() as usize]
                .get_neighbours()
                .into_iter()
                .filter(|cell_id| grid.grid[*cell_id as usize].get_linked_cells().is_empty())
                .collect();

            if !unvisited_neightbours.is_empty() {
                let random_neighbour = unvisited_neightbours
                    [rand::thread_rng().gen_range(0..unvisited_neightbours.len())];
                grid.link_cells(current_cell_id.unwrap(), random_neighbour);
                current_cell_id = Some(random_neighbour);
            } else {
                current_cell_id = None;
                for cell in grid.grid.to_owned() {
                    let visited_neightbours: Vec<i32> = grid.grid[cell.id as usize]
                        .get_neighbours()
                        .into_iter()
                        .filter(|cell_id| {
                            !grid.grid[*cell_id as usize].get_linked_cells().is_empty()
                        })
                        .collect();
                    if cell.get_linked_cells().is_empty() && !visited_neightbours.is_empty() {
                        current_cell_id = Some(cell.id);
                        let random_neighbour = visited_neightbours
                            [rand::thread_rng().gen_range(0..visited_neightbours.len())];
                        grid.link_cells(current_cell_id.unwrap(), random_neighbour);
                        break;
                    }
                }
            }
        }
    }

    fn get_name(&self) -> String {
        "Hunt and kill".to_owned()
    }
}
