use super::{cell::Cell, coord};

const MAX_WIDTH: usize = 26;
const MAX_HEIGHT: usize = 13;

pub struct Grid {
    cells: [Cell; MAX_WIDTH * MAX_HEIGHT],
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Self {
        Grid {
            cells: [0; MAX_WIDTH * MAX_HEIGHT],
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Cell {
        if self.is_in_bounds(x, y) {
            self.cells[x as usize + self.width as usize * y as usize]
        } else {
            panic!("Cell out of bounds");
        }
    }

    pub fn is_in_bounds(&self, x: u8, y: u8) -> bool {
        x < self.width && y < self.height
    }

    pub fn is_in_bounds_coord(&self, coord: coord::Coord) -> bool {
        self.is_in_bounds(coord::x(coord), coord::y(coord))
    }

    pub fn get_cell_from_coord(&self, coord: coord::Coord) -> Cell {
        self.get_cell(coord::x(coord), coord::y(coord))
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: Cell) {
        self.cells[x as usize + self.width as usize * y as usize] = cell;
    }
}

#[cfg(test)]
mod tests {
    use crate::game_entities::cell;

    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        assert_eq!(cell::is_obstacle(grid.get_cell(0, 0)), true);
        assert_eq!(cell::is_obstacle(grid.get_cell(1, 1)), false);
        assert_eq!(cell::is_obstacle(grid.get_cell(2, 2)), false);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let grid = Grid::new(3, 3);

        grid.get_cell(3, 3);
    }

    #[test]
    fn test_is_in_bounds() {
        let grid = Grid::new(3, 3);

        assert_eq!(grid.is_in_bounds(0, 0), true);
        assert_eq!(grid.is_in_bounds(2, 2), true);
        assert_eq!(grid.is_in_bounds(3, 3), false);
    }

    #[test]
    fn test_is_in_bounds_coord() {
        let grid = Grid::new(3, 3);

        assert_eq!(grid.is_in_bounds_coord(coord::new(0, 0)), true);
        assert_eq!(grid.is_in_bounds_coord(coord::new(2, 2)), true);
        assert_eq!(grid.is_in_bounds_coord(coord::new(3, 3)), false);
    }

    #[test]
    fn test_get_cell_from_coord() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        assert_eq!(
            cell::is_obstacle(grid.get_cell_from_coord(coord::new(0, 0))),
            true
        );
        assert_eq!(
            cell::is_obstacle(grid.get_cell_from_coord(coord::new(1, 1))),
            false
        );
        assert_eq!(
            cell::is_obstacle(grid.get_cell_from_coord(coord::new(2, 2))),
            false
        );
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        assert_eq!(cell::is_obstacle(grid.get_cell(0, 0)), true);
    }

    #[test]
    fn test_set_cell_from_coord() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        assert_eq!(cell::is_obstacle(grid.get_cell(0, 0)), true);
    }

    #[test]
    fn test_set_cell_from_coord_out_of_bounds() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        assert_eq!(cell::is_obstacle(grid.get_cell(0, 0)), true);
    }
}
