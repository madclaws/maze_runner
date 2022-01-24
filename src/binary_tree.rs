/*
    Maze generation using Binary Tree Algorithm
*/
use rand::Rng;

use crate::grid::*;
pub struct BinaryTree {}

impl Algorithm for BinaryTree {
    fn on(&self, grid: &mut Grid) {
        for cell in grid.grid.to_owned() {
            let mut neighbours: Vec<i32> = Vec::new();
            if let Some(cell_id) = cell.north {
                neighbours.push(cell_id);
            }
            if let Some(cell_id) = cell.east {
                neighbours.push(cell_id);
            }
            if !neighbours.is_empty() {
                let index = rand::thread_rng().gen_range(0..neighbours.len());
                let selected_neighbour = neighbours[index];
                grid.link_cells(cell.id, selected_neighbour);
            }
        }
    }

    fn get_name(&self) -> String {
        "BinaryTree".to_owned()
    }
}
