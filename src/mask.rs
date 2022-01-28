/* 
        Mask module
*/

pub struct Mask{
    pub rows: i32,
    pub cols: i32,
    pub grid: Vec<bool>
}

impl Mask {
    pub fn new(row: i32, col: i32) -> Self {
        Mask{rows: row, cols: col, grid: vec![true; (row * col) as usize]}
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
                return index as i32
            }
        }
        return -1
    }
}