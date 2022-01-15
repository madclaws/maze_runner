use std::env;

mod cell;
mod grid;
mod binary_tree;
mod sidewinder;
mod distances;

use macroquad::prelude::*;
use grid::*;

#[macroquad::main(conf())]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Mazes mazes everywhere..\n\n");

    let maze_name = &args[1].parse::<u32>().unwrap();
    let mut grid = Grid::new(10, 10);
    grid.configure_cells();        
    match maze_name {
        1 => {
            println!("Hydrogen - Binary tree\n");
            binary_tree::on(&mut grid);
            
            // grid.distances(0);
            grid.render();
        },
        2 => {
            println!("Helium - Sidewinder\n");
            sidewinder::on(&mut grid);
            grid.render();
        },
        3 => {
            println!("Lithium - Macroquad rendered sidewinder\n");
            sidewinder::on(&mut grid);
            loop {
                clear_background(BLACK);
                render(&grid);
                next_frame().await
            }
        },
        _ => panic!("Maze doesn't exist or element doesn't exist")
    }

   
}


fn render(grid: &Grid) {
    let cell_size: i32 = 50;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let x1 = (col * cell_size) as f32;
            let y1 = (row * cell_size) as f32;
            let x2 = ((col + 1) * cell_size) as f32;
            let y2 = ((row + 1) * cell_size) as f32;

            let index = (row * grid.cols) + col;
            
            if grid.grid[index as usize].north.is_none() {
                draw_line(x1, y1, x2, y1, 5.0, RED);
            }

            if grid.grid[index as usize].west.is_none() {
                draw_line(x1, y1, x1, y2, 5.0, RED);
            }

            if let Some(east_cell_id) = grid.grid[index as usize].east {
                if !grid.grid[index as usize].is_linked(east_cell_id) {
                    draw_line(x2, y1, x2, y2, 5.0, RED);
                }
            } else {
                draw_line(x2, y1, x2, y2, 5.0, RED);
            }

            if let Some(south_cell_id) = grid.grid[index as usize].south {
                if !grid.grid[index as usize].is_linked(south_cell_id) {
                    draw_line(x1, y2, x2, y2, 5.0, RED);
                }
            } else {
                draw_line(x1, y2, x2, y2, 5.0, RED);
            }

        }
    }
}

fn conf() -> Conf {
    Conf{
        window_title: String::from("Maze Runner"),
        window_width: 10 * (50 + 13),
        window_height: 10 * (50 + 13),
        fullscreen: false,
        ..Default::default()
    }
}