use super::{cell::Cell, coord};

pub struct Grid<'a> {
    cells: Vec<Cell<'a>>,
    width: u8,
    height: u8,
}

impl<'a> Grid<'a> {
    pub fn new(width: u8, height: u8) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new_wall(coord::new(x, y)));
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> &Cell {
        if x < self.width && y < self.height {
            &self.cells[x as usize + self.width as usize * y as usize]
        } else {
            panic!("Cell out of bounds");
        }
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: &Cell<'a>) {
        self.cells[x as usize + self.width as usize * y as usize].assign_new_cell(cell);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.get_cell(0, 0).is_obstacle(), true);
        assert_eq!(grid.get_cell(1, 1).is_obstacle(), true);
        assert_eq!(grid.get_cell(2, 2).is_obstacle(), true);
    }
}
