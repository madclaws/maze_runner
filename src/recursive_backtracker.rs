/*
    Implementing recursive backtracker algorithm
*/

use crate::grid::*;
use rand::Rng;

pub struct RecursiveBacktracker;

impl Algorithm for RecursiveBacktracker {
    fn on(&self, grid: &mut Grid) {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(rand::thread_rng().gen_range(0..grid.grid.len()) as i32);
        while !stack.is_empty() {
            let current_cell_id: Option<&i32> = stack.last();
            let unvisited_neightbours: Vec<i32> = grid.grid[*current_cell_id.unwrap() as usize]
                .get_neighbours()
                .into_iter()
                .filter(|cell_id| grid.grid[*cell_id as usize].get_linked_cells().is_empty())
                .collect();
            if unvisited_neightbours.is_empty() {
                stack.pop();
            } else {
                let random_neighbour = unvisited_neightbours
                    [rand::thread_rng().gen_range(0..unvisited_neightbours.len())];
                grid.link_cells(*current_cell_id.unwrap(), random_neighbour);
                stack.push(random_neighbour);
            }
        }
    }

    fn get_name(&self) -> String {
        "Recursive Backtracker".to_owned()
    }
}
