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
#[derive(Debug)]
pub struct Cell{
    id: u32,
    row: u32,
    col: u32,
    north: Option<u32>  ,
    south: Option<u32> ,
    west: Option<u32>,
    east: Option<u32>,
    links: HashMap<u32, bool>
}

impl Cell {
    pub fn new(id: u32, row: u32, col: u32) -> Cell {
        Cell{id: id, row: row, col: col, north: None, south: None,
            east: None, west: None, links: HashMap::new()}
    }

    pub fn link(&mut self, linked_cell: &mut Cell, is_bidirectional: bool) {
        self.links.insert(linked_cell.id, true);
        if is_bidirectional { // The flag stops recursive calling.
            linked_cell.link(self, false);
        }
    }

    pub fn unlink(&mut self, unlinked_cell: &mut Cell, is_bidirectional: bool) {
        self.links.remove(&unlinked_cell.id);
        if is_bidirectional { // The flag stops recursive calling.
            unlinked_cell.unlink(self, false);
        }
    }

    pub fn get_linked_cells(&self) -> Vec<u32> {
        let linked_keys: Vec<u32> = self.links.keys().map(|x| *x).collect();
        linked_keys
    }

    pub fn is_linked(&self, cell: &Cell) -> bool {
        self.links.contains_key(&cell.id)
    }

    pub fn get_neighbours(&self) -> Vec<u32> {
        let mut neighbours: Vec<u32> = Vec::new();
        if let Some(north) = self.north {
            neighbours.push(north);
        }
        if let Some(south) = self.south {
            neighbours.push(south);
        }if let Some(west) = self.west {
            neighbours.push(west);
        }if let Some(east) = self.east {
            neighbours.push(east);
        }
        neighbours
    }
    
}

#[cfg(test)]
mod tests{
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
    fn linking_and_unlinking_cells(){
        let mut cell_a = Cell::new(0, 0, 0);
        let mut cell_b = Cell::new(1, 0, 1);
        cell_a.link(&mut cell_b, true);
        assert_eq!(cell_a.links.contains_key(&1), true);
        assert_eq!(cell_b.links.contains_key(&0), true);
        cell_b.unlink(&mut cell_a, true);
        assert_ne!(cell_a.links.contains_key(&1), true);
        assert_ne!(cell_b.links.contains_key(&0), true);
    }

    #[test]
    fn getting_linked_cells() {
        let mut cell_a = Cell::new(0, 0, 0);
        let mut cell_b = Cell::new(1, 0, 1);
        let mut cell_c = Cell::new(2, 1, 0);
        cell_a.link(&mut cell_b, true);
        cell_a.link(&mut cell_c, true);
        let linked_cells = cell_a.get_linked_cells();
        assert_eq!(linked_cells.len(), 2);
    }

    #[test]
    fn is_linked() {
        let mut cell_a = Cell::new(0, 0, 0);
        let mut cell_b = Cell::new(1, 0, 1);
        let mut cell_c = Cell::new(2, 1, 0);
        cell_a.link(&mut cell_b, true);
        cell_a.link(&mut cell_c, true);
        assert_eq!(cell_a.is_linked(&cell_b), true);
        cell_c.unlink(&mut cell_a, true);
        assert_eq!(cell_c.is_linked(&cell_a), false);
    }

    #[test]
    fn getting_neighbours() {
        let cell_a = Cell::new(0, 0, 0);
        assert_eq!(cell_a.get_neighbours().len(), 0);
    }
}