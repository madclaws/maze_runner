/* 
    Maze generation using Sidewinder algorithm
*/

use crate::grid::*;
use crate::cell::*;
use rand::Rng;

pub fn on(grid: &mut Grid) -> &mut Grid {
    let owned_grid = grid.grid.to_owned();
    for i in 0..grid.rows {
        let mut run: Vec<&Cell> = Vec::new();
        for j in 0..grid.cols {
            let cell = &owned_grid[(i * grid.cols + j) as usize];
            run.push(cell);
            let at_eastern_boundary = cell.east.is_none();
            let at_nothern_boundary = cell.north.is_none();

            let should_close_out = at_eastern_boundary || (!at_nothern_boundary && 
                rand::thread_rng().gen_range(0..2) == 0);
            
            if should_close_out {
                let index = rand::thread_rng().gen_range(0..run.len());
                let member = run[index];
                if let Some(north_cell_id) = member.north {
                    grid.link_cells(member.id, north_cell_id)
                }
                run.clear();
            } else {
                grid.link_cells(cell.id, cell.east.unwrap())
            }
        }
    }
    grid
}