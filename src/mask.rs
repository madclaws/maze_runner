/*
        Mask module
*/

use std::fs;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub struct Mask {
    pub rows: i32,
    pub cols: i32,
    pub grid: Vec<bool>,
}

impl Mask {
    pub fn new(row: i32, col: i32) -> Self {
        Mask {
            rows: row,
            cols: col,
            grid: vec![true; (row * col) as usize],
        }
    }

    pub fn apply_pattern(&mut self) {
        let pattern: String =
            fs::read_to_string("/home/madclaws/labs/maze_runner/mazes/mask_a.txt")
                .expect("Unable to load the pattern file");
        let pattern_lines: Vec<&str> = pattern.split('\n').collect();
        let mut pattern_cell_count = 0;
        for pattern_line in &pattern_lines {
            let cell_patterns: Vec<&str> = pattern_line.split(' ').collect();
            for cell_pattern in &cell_patterns {
                if cell_pattern == &"X" {
                    self.grid[pattern_cell_count as usize] = false;
                }
                pattern_cell_count += 1;
            }
        }
    }

    pub fn apply_pattern_from_image(&mut self) {
        let img = ImageReader::open("/home/madclaws/labs/maze_runner/mazes/maze_text.png").unwrap().decode().unwrap();
        let pixel_vec = img.into_rgb32f().clone();

        for (index, pixel) in pixel_vec.iter().step_by(3).enumerate() {
            if *pixel == 0.0 {
                self.grid[index as usize] = false;
            }   
        }
    }

    pub fn set(&mut self, cell: (i32, i32), status: bool) {
        self.grid[(cell.0 * self.cols + cell.1) as usize] = status;
    }

    pub fn get(&self, cell: (i32, i32)) -> bool {
        self.grid[(cell.0 * self.cols + cell.1) as usize]
    }

    pub fn get_random_index(&self) -> i32 {
        for (index, status) in self.grid.iter().enumerate() {
            if *status {
                return index as i32;
            }
        }
        return -1
    }
}
