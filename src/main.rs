use std::env;
mod aldous_broder;
mod binary_tree;
mod cell;
mod distances;
mod grid;
mod hunt_and_kill;
mod sidewinder;
mod wilsons;
use distances::*;
use grid::*;
use hunt_and_kill::*;
use macroquad::prelude::*;
mod deadend_count;
mod recursive_backtracker;
mod mask;

enum RenderMode {
    Walls,
    Background,
}

#[macroquad::main(conf())]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Mazes mazes everywhere..\n\n");

    let maze_name = &args[1].parse::<i32>().unwrap();
    let mut grid = Grid::new(10, 10);
    grid.configure_cells();
    match maze_name {
        0 => {
            deadend_count::count_deadends(13, 13);
        }
        1 => {
            println!("Hydrogen - Binary tree\n");
            binary_tree::BinaryTree {}.on(&mut grid);

            grid.render();
        }
        2 => {
            println!("Helium - Sidewinder\n");
            sidewinder::SideWinder {}.on(&mut grid);
            grid.render();
        }
        3 => {
            println!("Lithium - Macroquad rendered sidewinder\n");
            binary_tree::BinaryTree {}.on(&mut grid);
            loop {
                clear_background(BLACK);

                render(
                    &grid,
                    RenderMode::Walls,
                    &Distances::new(0),
                    RED,
                    0.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        4 => {
            println!("Beryllium - Maze solver using Dijkstra's algorithm\n");
            binary_tree::BinaryTree {}.on(&mut grid);
            let distances = grid.distances(0);
            let breadcrumbs = grid.breadcrumbs(90, 0, &distances);
            loop {
                clear_background(BLACK);
                render(&grid, RenderMode::Walls, &distances, BLUE, 0.0, 0.0, true);
                render(
                    &grid,
                    RenderMode::Walls,
                    &breadcrumbs,
                    BLUE,
                    600.0,
                    0.0,
                    true,
                );
                next_frame().await
            }
        }
        5 => {
            println!("Boron - Hard maze using Dijkstra's algorithm\n");
            let mut grid = Grid::new(10, 10);
            grid.configure_cells();
            binary_tree::BinaryTree {}.on(&mut grid);
            let root = 0;
            let distances = grid.distances(root); // 0 is the starting_point

            // Finding the fathest cell from the root(0).
            let (farthest_cell, _farthest_distance) = distances.get_most_distant(root);

            // Find distances of cells relative to the farthest cell.
            let farthest_distances = grid.distances(farthest_cell);

            let (new_goal_cell, _goal_distance) =
                farthest_distances.get_most_distant(farthest_cell);

            // Finding the path reversely
            let breadcrumbs = grid.breadcrumbs(new_goal_cell, farthest_cell, &farthest_distances);

            loop {
                clear_background(BLACK);
                render(
                    &grid,
                    RenderMode::Walls,
                    &Distances::new(0),
                    PINK,
                    0.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &breadcrumbs,
                    PINK,
                    600.0,
                    0.0,
                    true,
                );
                // render(&grid, &farthest_distances, BLUE, 1000.0, 0.0);

                next_frame().await
            }
        }
        6 => {
            println!("Carbon - Coloring the maze\n");
            let mut grid = Grid::new(13, 13);
            grid.configure_cells();
            sidewinder::SideWinder {}.on(&mut grid);

            let mut binary_grid = Grid::new(13, 13);
            binary_grid.configure_cells();
            binary_tree::BinaryTree {}.on(&mut binary_grid);

            let distances = grid.distances(84);
            let distances_binary = binary_grid.distances(84);

            loop {
                clear_background(GRAY);
                render(
                    &binary_grid,
                    RenderMode::Background,
                    &distances_binary,
                    RED,
                    0.0,
                    0.0,
                    false,
                );
                render(
                    &binary_grid,
                    RenderMode::Walls,
                    &distances_binary,
                    BLACK,
                    0.0,
                    0.0,
                    false,
                );

                render(
                    &grid,
                    RenderMode::Background,
                    &distances,
                    BLUE,
                    800.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    BLACK,
                    800.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        7 => {
            println!("Nitrogen - Aldous Brorder algorithm\n");
            let mut grid = Grid::new(13, 13);
            grid.configure_cells();
            aldous_broder::AldousBroder {}.on(&mut grid);

            let distances = grid.distances(84);

            loop {
                // clear_background(Color::new(1.0, 204.0/255.0, 203.0/255.0, 1.0));
                clear_background(BLACK);

                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    DARKGREEN,
                    0.0,
                    0.0,
                    false,
                );

                render(
                    &grid,
                    RenderMode::Background,
                    &distances,
                    GREEN,
                    800.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    BLACK,
                    800.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        8 => {
            println!("Oxygen - Wilson's Algorithm");
            let mut grid = Grid::new(13, 13);
            grid.configure_cells();
            wilsons::Wilsons {}.on(&mut grid);
            let distances = grid.distances(84);
            loop {
                clear_background(BLACK);
                render(&grid, RenderMode::Walls, &distances, GOLD, 0.0, 0.0, false);

                render(
                    &grid,
                    RenderMode::Background,
                    &distances,
                    GOLD,
                    800.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    BLACK,
                    800.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        9 => {
            println!("Flourine - Hunt and kill Algorithm");
            let mut grid = Grid::new(13, 13);
            grid.configure_cells();
            HuntAndKill {}.on(&mut grid);
            let distances = grid.distances(84);
            loop {
                clear_background(BLACK);
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    SKYBLUE,
                    0.0,
                    0.0,
                    false,
                );

                render(
                    &grid,
                    RenderMode::Background,
                    &distances,
                    SKYBLUE,
                    800.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    BLACK,
                    800.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        10 => {
            println!("Neon - Recursive backtracker Algorithm");
            let mut grid = Grid::new(13, 13);
            grid.configure_cells();
            recursive_backtracker::RecursiveBacktracker {}.on(&mut grid);
            let distances = grid.distances(84);
            let neon_color = Color::new(1.0, 0.0, 153.0 / 255.0, 1.0);
            loop {
                clear_background(BLACK);
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    neon_color,
                    0.0,
                    0.0,
                    false,
                );

                render(
                    &grid,
                    RenderMode::Background,
                    &distances,
                    neon_color,
                    800.0,
                    0.0,
                    false,
                );
                render(
                    &grid,
                    RenderMode::Walls,
                    &distances,
                    BLACK,
                    800.0,
                    0.0,
                    false,
                );
                next_frame().await
            }
        }
        _ => panic!("Maze doesn't exist or element doesn't exist"),
    }
}

fn render(
    grid: &Grid,
    render_mode: RenderMode,
    distances: &Distances,
    color: Color,
    h_offset: f32,
    v_offset: f32,
    render_text: bool,
) {
    let cell_size: i32 = 50;
    let thickness = 3.0;
    let line_color = color;
    let x_offset = h_offset;
    let y_offset = v_offset;
    let mut max_distance = 0;
    if matches!(render_mode, RenderMode::Background) {
        let (_fartherst_cell, farthest_distance) = distances.get_most_distant(distances.root);
        max_distance = farthest_distance;
    }
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            match render_mode {
                RenderMode::Walls => {
                    let wall_config = (
                        row, col, cell_size, grid, distances, x_offset, y_offset, thickness,
                        line_color,
                    );
                    render_walls(wall_config, render_text);
                }
                RenderMode::Background => {
                    let rect_config = (
                        row,
                        col,
                        cell_size,
                        grid,
                        distances,
                        x_offset,
                        y_offset,
                        max_distance,
                        color,
                    );
                    render_rects(rect_config);
                }
            }
        }
    }
}

fn render_walls(
    wall_config: (i32, i32, i32, &Grid, &Distances, f32, f32, f32, Color),
    render_text: bool,
) {
    let cell_size = wall_config.2;
    let thickness = wall_config.7;
    let line_color = wall_config.8;
    let x_offset = wall_config.5;
    let y_offset = wall_config.6;
    let col = wall_config.1;
    let row = wall_config.0;
    let grid = wall_config.3;
    let distances = wall_config.4;

    let x1 = (col * cell_size) as f32;
    let y1 = (row * cell_size) as f32;
    let x2 = ((col + 1) * cell_size) as f32;
    let y2 = ((row + 1) * cell_size) as f32;

    let index = (row * grid.cols) + col;

    if grid.grid[index as usize].north.is_none() {
        draw_line(
            x1 + x_offset,
            y1 + y_offset,
            x2 + x_offset,
            y1 + y_offset,
            thickness,
            line_color,
        );
    }

    if grid.grid[index as usize].west.is_none() {
        draw_line(
            x1 + x_offset,
            y1 + y_offset,
            x1 + x_offset,
            y2 + y_offset,
            thickness,
            line_color,
        );
    }

    if let Some(east_cell_id) = grid.grid[index as usize].east {
        if !grid.grid[index as usize].is_linked(east_cell_id) {
            draw_line(
                x2 + x_offset,
                y1 + y_offset,
                x2 + x_offset,
                y2 + y_offset,
                thickness,
                line_color,
            );
        }
    } else {
        draw_line(
            x2 + x_offset,
            y1 + y_offset,
            x2 + x_offset,
            y2 + y_offset,
            thickness,
            line_color,
        );
    }

    if let Some(south_cell_id) = grid.grid[index as usize].south {
        if !grid.grid[index as usize].is_linked(south_cell_id) {
            draw_line(
                x1 + x_offset,
                y2 + y_offset,
                x2 + x_offset,
                y2 + y_offset,
                thickness,
                line_color,
            );
        }
    } else {
        draw_line(
            x1 + x_offset,
            y2 + y_offset,
            x2 + x_offset,
            y2 + y_offset,
            thickness,
            line_color,
        );
    }
    let mut dist = String::from("");
    if render_text {
        if let Some(valid_dist) = distances.get_distance(index) {
            dist = valid_dist.to_string();
        }
    }
    draw_text_ex(
        &dist[..],
        x1 + x_offset + (cell_size - 13) as f32 / 2.0,
        y1 + y_offset + (cell_size + 13) as f32 / 2.0,
        TextParams {
            font_size: 12,
            ..Default::default()
        },
    );
}

fn render_rects(bg_config: (i32, i32, i32, &Grid, &Distances, f32, f32, u32, Color)) {
    let cell_size = bg_config.2;
    let col = bg_config.1;
    let row = bg_config.0;
    let grid = bg_config.3;
    let distances = bg_config.4;
    let max_distance = bg_config.7;
    let x_offset = bg_config.5;
    let y_offset = bg_config.6;

    let x1 = (col * cell_size) as f32;
    let y1 = (row * cell_size) as f32;
    let index = (row * grid.cols) + col;

    if let Some(color) = distances.get_background_color(max_distance, index, bg_config.8) {
        draw_rectangle(x1 + x_offset, y1 + y_offset, 50.0, 50.0, color);
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Maze Runner"),
        window_width: 100 * (50 + 13),
        window_height: 100 * (50 + 13),
        fullscreen: false,
        ..Default::default()
    }
}
