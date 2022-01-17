use std::env;

mod cell;
mod grid;
mod binary_tree;
mod sidewinder;
mod distances;

use macroquad::prelude::*;
use grid::*;
use distances::*;

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
            binary_tree::on(&mut grid);
            // let distances = grid.distances(0);
            // let breadcrumbs = grid.breadcrumbs(90, 0, &distances);
            loop {
                clear_background(BLACK);
                render(&grid, &Distances::new(0), RED, 0.0, 0.0);
                next_frame().await
            }
        },
        4 => {
            println!("Beryllium - Maze solver using Dijkstra's algorithm\n");
            binary_tree::on(&mut grid);
            let distances = grid.distances(0);
            let breadcrumbs = grid.breadcrumbs(90, 0, &distances);
            loop {
                clear_background(BLACK);
                render(&grid, &distances, BLUE, 0.0, 0.0);
                render(&grid, &breadcrumbs, BLUE, 600.0, 0.0);
                next_frame().await
            }
        },
        5 => {
            println!("Boron - Hard maze using Dijkstra's algorithm\n");
            let mut grid = Grid::new(10, 10);
            grid.configure_cells();        
            binary_tree::on(&mut grid);
            let root = 0;
            let distances = grid.distances(root); // 0 is the starting_point
            
            // Finding the fathest cell from the root(0).
            let (farthest_cell, _farthest_distance) = distances.get_most_distant(root);

            // Find distances of cells relative to the farthest cell.
            let farthest_distances = grid.distances(farthest_cell);

            let (new_goal_cell, _goal_distance) = farthest_distances.get_most_distant(farthest_cell);

            // Finding the path reversely
            let breadcrumbs = grid.breadcrumbs(new_goal_cell, farthest_cell, &farthest_distances);
            
            loop {
                clear_background(BLACK);
                render(&grid, &Distances::new(0), PINK, 0.0, 0.0);
                render(&grid, &breadcrumbs, PINK, 600.0, 0.0);
                // render(&grid, &farthest_distances, BLUE, 1000.0, 0.0);

                next_frame().await
            }
        },
        _ => panic!("Maze doesn't exist or element doesn't exist")
    }

   
}   


fn render(grid: &Grid, distances: &Distances, color: Color, h_offset: f32, v_offset: f32) {
    let cell_size: i32 = 50;
    let thickness = 5.0;
    let line_color = color;
    let x_offset = h_offset;
    let y_offset = v_offset;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let x1 = (col * cell_size) as f32;
            let y1 = (row * cell_size) as f32;
            let x2 = ((col + 1) * cell_size) as f32;
            let y2 = ((row + 1) * cell_size) as f32;

            let index = (row * grid.cols) + col;
            
            if grid.grid[index as usize].north.is_none() {
                draw_line(x1 + x_offset, y1 + y_offset, x2 + x_offset, y1 + y_offset, thickness, line_color);
            }

            if grid.grid[index as usize].west.is_none() {
                draw_line(x1 + x_offset, y1 + y_offset, x1 + x_offset, y2 + y_offset, thickness, line_color);
            }

            if let Some(east_cell_id) = grid.grid[index as usize].east {
                if !grid.grid[index as usize].is_linked(east_cell_id) {
                    draw_line(x2 + x_offset, y1 + y_offset, x2 + x_offset, y2 + y_offset, thickness, line_color);
                }
            } else {
                draw_line(x2 + x_offset, y1 + y_offset, x2 + x_offset, y2 + y_offset, thickness, line_color);
            }

            if let Some(south_cell_id) = grid.grid[index as usize].south {
                if !grid.grid[index as usize].is_linked(south_cell_id) {
                    draw_line(x1 + x_offset, y2 + y_offset, x2 + x_offset, y2 + y_offset, thickness, line_color);
                }
            } else {
                draw_line(x1 + x_offset, y2 + y_offset, x2 + x_offset, y2 + y_offset, thickness, line_color);
            }
            let mut dist = String::from("");
            if distances.get_total_distances() > 1 {
                if let Some(valid_dist) = distances.get_distance(index) {
                    dist = valid_dist.to_string();
                }
            }
            draw_text_ex(&dist[..], x1 + x_offset + (cell_size - 13) as f32 / 2.0, y1 + y_offset + (cell_size + 13) as f32 / 2.0, TextParams{
                font_size: 12, ..Default::default()
            });

        }
    }
}

fn conf() -> Conf {
    Conf{
        window_title: String::from("Maze Runner"),
        window_width: 100 * (50 + 13),
        window_height: 100 * (50 + 13),
        fullscreen: false,
        ..Default::default()
    }
}