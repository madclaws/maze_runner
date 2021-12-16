/* 
    Maze generation using Binary Tree Algorithm
*/
use rand::Rng;

pub struct BinaryTree {

}
use crate::grid::*;

impl BinaryTree {
    fn on(grid: &mut Grid) -> &mut Grid {
        grid.each_cell_mut().for_each(|cell| {
            let mut neighbours: Vec<i32> = Vec::new();
            if let Some(cell_id) = cell.north {
                neighbours.push(cell_id);
            }
            if let Some(cell_id) = cell.east {
                neighbours.push(cell_id);
            }
            let index = rand::thread_rng().gen_range(0..neighbours.len());
            let selected_neighbour = neighbours[index];
            // cell.link() 
        });

        grid
    }
}