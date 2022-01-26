/*
    Counting the deadends for all algorithms
*/

use crate::aldous_broder::*;
use crate::binary_tree::*;
use crate::grid::*;
use crate::hunt_and_kill::*;
use crate::recursive_backtracker::*;
use crate::sidewinder::*;
use crate::wilsons::*;

use std::collections::HashMap;

pub fn count_deadends(maze_size: i32, iterations: i32) {
    // Trait objects
    let mut algorithms: Vec<Box<dyn Algorithm>> = vec![
        Box::new(AldousBroder {}),
        Box::new(BinaryTree {}),
        Box::new(SideWinder {}),
        Box::new(Wilsons {}),
        Box::new(HuntAndKill {}),
        Box::new(RecursiveBacktracker {}),
    ];

    let mut deadend_average: HashMap<String, i32> = HashMap::new();

    for algorithm in &algorithms {
        let mut deadend_cells_count: Vec<i32> = Vec::new();
        for _iteration in 0..iterations {
            let mut grid = Grid::new(maze_size as i32, maze_size as i32);
            grid.configure_cells();
            algorithm.on(&mut grid);
            let deadend_cells = grid.get_deadend_cells();
            deadend_cells_count.push(deadend_cells.len() as i32);
        }
        let average = deadend_cells_count.iter().sum::<i32>() / deadend_cells_count.len() as i32;
        deadend_average.insert(algorithm.get_name(), average);
    }

    println!(
        "Average dead-ends per {maze_size} * {maze_size} maze ({}) cells\n",
        maze_size * maze_size
    );

    algorithms.sort_by(|a, b| deadend_average[&b.get_name()].cmp(&deadend_average[&a.get_name()]));

    for algorithm in &algorithms {
        let percentage = deadend_average[&algorithm.get_name()] * 100 / (maze_size * maze_size);
        println!(
            "{}  \t: {}/{} ({}%)",
            algorithm.get_name(),
            deadend_average[&algorithm.get_name()],
            maze_size * maze_size,
            percentage
        );
    }
}
