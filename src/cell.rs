/*
    Cell module, which implements Cell struct and its functionalities.
*/
use std::collections::HashMap;

/// `Cell` is the atomic entity in maze, a matrix of cells makes a `Grid`.
///
/// * id - The unique id of cell in a Grid.
/// * row - Row number of the Cell in a Grid
/// * col - Column number of the Cell in a Grid.
/// * north - Id of the northern neighbour of the Cell
/// * south - Id of the southern neighbour of the Cell.
/// * west - Id of the western neighbour of the Cell.
/// * east - Id of the eastern neighbour of the Cell.
/// * links - A mapping of open links (joined by a passage) to other cells.
#[derive(Debug, Clone)]
pub struct Cell {
    pub id: i32,
    pub row: i32,
    pub col: i32,
    pub north: Option<i32>,
    pub south: Option<i32>,
    pub west: Option<i32>,
    pub east: Option<i32>,
    links: HashMap<i32, bool>,
}

impl Cell {
    pub fn new(id: i32, row: i32, col: i32) -> Cell {
        Cell {
            id,
            row,
            col,
            north: None,
            south: None,
            east: None,
            west: None,
            links: HashMap::new(),
        }
    }

    pub fn link(&mut self, linked_cell_id: i32) {
        self.links.insert(linked_cell_id, true);
    }

    /// Unlinks 2 cell bidirectionally
    pub fn unlink(&mut self, unlinked_cell_id: i32) {
        self.links.remove(&unlinked_cell_id);
    }

    pub fn get_linked_cells(&self) -> Vec<i32> {
        let linked_keys: Vec<i32> = self.links.keys().copied().collect();
        linked_keys
    }

    pub fn is_linked(&self, cell_id: i32) -> bool {
        self.links.contains_key(&cell_id)
    }


    fn _get_neighbours(&self) -> Vec<i32> {
        let mut neighbours: Vec<i32> = Vec::new();
        if let Some(north) = self.north {
            neighbours.push(north);
        }
        if let Some(south) = self.south {
            neighbours.push(south);
        }
        if let Some(west) = self.west {
            neighbours.push(west);
        }
        if let Some(east) = self.east {
            neighbours.push(east);
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn creating_new_cells() {
        let cell_a = Cell::new(0, 0, 0);
        assert_eq!(cell_a.id, 0);
        assert_eq!(cell_a.row, 0);
        assert_eq!(cell_a.col, 0);
        assert_eq!(cell_a.links.is_empty(), true);
    }

    #[test]
    fn linking_and_unlinking_cells() {
        let mut cell_a = Cell::new(0, 0, 0);
        cell_a.link(1);
        assert_eq!(cell_a.links.contains_key(&1), true);
        // assert_eq!(cell_b.links.contains_key(&0), true);
        // cell_b.unlink(0);
        // assert_ne!(cell_a.links.contains_key(&1), true);
        // assert_ne!(cell_b.links.contains_key(&0), true);
    }

    #[test]
    fn getting_linked_cells() {
        let mut cell_a = Cell::new(0, 0, 0);
        let mut cell_b = Cell::new(1, 0, 1);
        let mut cell_c = Cell::new(2, 1, 0);
        cell_a.link(1);
        cell_a.link(2);
        let linked_cells = cell_a.get_linked_cells();
        assert_eq!(linked_cells.len(), 2);
    }

    #[test]
    fn is_linked() {
        let mut cell_a = Cell::new(0, 0, 0);
        let cell_b = Cell::new(1, 0, 1);
        let mut cell_c = Cell::new(2, 1, 0);
        cell_a.link(1);
        cell_a.link(2);
        assert_eq!(cell_a.is_linked(1), true);
        cell_c.unlink(0);
        assert_eq!(cell_c.is_linked(0), false);
    }

    #[test]
    fn getting_neighbours() {
        let cell_a = Cell::new(0, 0, 0);
        assert_eq!(cell_a._get_neighbours().len(), 0);
    }
}
