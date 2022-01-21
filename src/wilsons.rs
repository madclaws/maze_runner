/* 
    Implementing Wilson's algorithm

    An unbiased random-walk maze, where slow start and fast end
*/

use rand::Rng;

use crate::grid::*;

pub fn on(grid: &mut Grid) -> &mut Grid {
    // Inititalization
    let mut unvisited_cells: Vec<i32> = (0..(grid.cols * grid.rows)).collect();
    let first_visited_cell = unvisited_cells.remove(rand::thread_rng().gen_range(0..unvisited_cells.len()));
    println!("FIRST VISITED CELL {first_visited_cell}");
    println!("TOTAL UNVISITED CELLS => {}", unvisited_cells.len());

    while !unvisited_cells.is_empty() {
        println!("TOTAL UNVISITED CELLS => {}", unvisited_cells.len());
        let mut cell = unvisited_cells[rand::thread_rng().gen_range(0..unvisited_cells.len())];
        let mut path: Vec<i32> = Vec::new();
        println!("CURRENT CELL => {cell}");
        while unvisited_cells.contains(&cell) {
            let neightbours = grid.grid[cell as usize].get_neighbours();
            cell = neightbours[rand::thread_rng().gen_range(0..neightbours.len())];
            println!("NEIGHBOUR CELL {cell}");
            if let Some(position) = path.iter().position(|cell_index| *cell_index == cell) {
                println!("LOOOOP => {position}");
                println!("CURRENT PATH => {path:?}");
                path.truncate(position + 1);    
                println!("TRUNCATED PATH => {path:?}");
            } else {
                path.push(cell);
            }
        }

        println!("REACHED VISITED CELL {cell}");
        println!("CYCLE PATH {path:?}");
        if path.len() > 1 {
            for index in 0..path.len() - 2 {
                if path.len() < 1 {
                    if let Some(path_cell_position) = unvisited_cells.iter().position(|cell| *cell == path[index]) {
                        unvisited_cells.remove(path_cell_position);
                    }
                } else {
                    grid.link_cells(path[index], path[index + 1]);
                    let path_cell_position = unvisited_cells.iter().position(|cell| *cell == path[index]).unwrap();
                    unvisited_cells.remove(path_cell_position);
                }
            }
        }
     
    }
    grid
}