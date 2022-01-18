/*
    Distance module, which implements Distance struct and its utilities.

    Distance module, helps in storing and fetching of the distances btw
    root cell and others, which we will then use for implementing Dijkstra's
    algorithm
*/

use macroquad::color::Color;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Distances {
    cells: HashMap<i32, u32>,
    pub root: i32,
}

impl Distances {
    pub fn new(root_cell_id: i32) -> Self {
        let mut cells: HashMap<i32, u32> = HashMap::new();
        cells.insert(root_cell_id, 0);
        Distances {
            cells,
            root: root_cell_id,
        }
    }

    pub fn get_distance(&self, cell_id: i32) -> Option<&u32> {
        self.cells.get(&cell_id)
    }

    pub fn set_distance(&mut self, cell_id: i32, distance: u32) -> Option<u32> {
        self.cells.insert(cell_id, distance)
    }

    pub fn get_most_distant(&self, cell_id: i32) -> (i32, u32) {
        let mut max_cell: i32 = cell_id;
        let mut max_distance = 0; // ofcourse as cell_id is the starting point

        for (cell_id, distance) in &self.cells {
            if distance > &max_distance {
                max_cell = *cell_id;
                max_distance = *distance;
            }
        }
        (max_cell, max_distance)
    }

    pub fn get_background_color(
        &self,
        max_distance: u32,
        cell_id: i32,
        color_range: Color,
    ) -> Option<Color> {
        if self.get_distance(cell_id).is_none() {
            return None;
        }
        let cell_distance = self.get_distance(cell_id).unwrap();
        let intensity = (max_distance - cell_distance) as f32 / max_distance as f32;
        let dark = f32::round(255.0 * intensity);
        let bright = 128.0 + f32::round(127.0 * intensity);

        if color_range.r == 0.9 {
            Some(Color::new(bright / 255.0, dark / 255.0, dark / 255.0, 1.0))
        } else if color_range.g == 0.89 {
            Some(Color::new(dark / 255.0, bright / 255.0, dark / 255.0, 1.0))
        } else {
            Some(Color::new(dark / 255.0, dark / 255.0, bright / 255.0, 1.0))
        }
    }

    pub fn get_total_distances(&self) -> u32 {
        self.cells.len() as u32
    }

    fn get_cells(&self) -> Vec<&i32> {
        self.cells.keys().collect::<Vec<&i32>>()
    }
}

#[cfg(test)]
#[test]
fn distances_basic_functions() {
    let mut distances = Distances::new(0);
    assert_eq!(distances.get_distance(0), Some(&0));
    distances.set_distance(1, 1);
    assert_eq!(distances.get_distance(1), Some(&1));
    assert_eq!(distances.get_distance(2), None);
    assert_eq!(distances.get_cells().len(), 2);
}
