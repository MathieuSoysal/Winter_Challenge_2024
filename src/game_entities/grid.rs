use std::collections::{HashMap, HashSet};

use super::{
    cell::{self, Cell},
    coord::{self, Coord},
    organ::{self, get_type, Organ},
    organ_type::OrganType,
};

const MAX_WIDTH: usize = 26;

const MAX_HEIGHT: usize = 13;

pub struct Grid {
    cells: [Cell; MAX_WIDTH * MAX_HEIGHT],
    cell_connections: HashMap<coord::Coord, HashSet<coord::Coord>>,
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Self {
        Grid {
            cell_connections: HashMap::new(),
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

    pub fn get_adjacent_coords(&self, coord: coord::Coord) -> Vec<coord::Coord> {
        let mut adjacents = Vec::new();

        let x = coord::x(coord);
        let y = coord::y(coord);

        if x > 0 && self.is_in_bounds(x - 1, y) {
            adjacents.push(coord::new(x - 1, y));
        }
        if self.is_in_bounds(x + 1, y) {
            adjacents.push(coord::new(x + 1, y));
        }
        if y > 0 && self.is_in_bounds(x, y - 1) {
            adjacents.push(coord::new(x, y - 1));
        }
        if self.is_in_bounds(x, y + 1) {
            adjacents.push(coord::new(x, y + 1));
        }
        adjacents
    }

    pub fn get_adjacent_cells(&self, coord: coord::Coord) -> Vec<Cell> {
        let mut adjacents = Vec::new();

        let x = coord::x(coord);
        let y = coord::y(coord);

        if x > 0 && self.is_in_bounds(x - 1, y) {
            adjacents.push(self.get_cell(x - 1, y));
        }
        if self.is_in_bounds(x + 1, y) {
            adjacents.push(self.get_cell(x + 1, y));
        }
        if y > 0 && self.is_in_bounds(x, y - 1) {
            adjacents.push(self.get_cell(x, y - 1));
        }
        if self.is_in_bounds(x, y + 1) {
            adjacents.push(self.get_cell(x, y + 1));
        }
        adjacents
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: Cell) {
        self.cells[x as usize + self.width as usize * y as usize] = cell;
    }

    fn get_one_adjacent_organ(&self, coord: Coord, owner: u8, root_coord: Coord) -> Coord {
        let x = coord::x(coord);
        let y = coord::y(coord);
        if x > 0 && cell::is_owned_and_rooted_by(self.get_cell(x - 1, y), owner, root_coord) {
            return coord::new(x - 1, y);
        }
        if cell::is_owned_and_rooted_by(self.get_cell(x + 1, y), owner, root_coord) {
            return coord::new(x + 1, y);
        }
        if y > 0 && cell::is_owned_and_rooted_by(self.get_cell(x, y - 1), owner, root_coord) {
            return coord::new(x, y - 1);
        }
        if cell::is_owned_and_rooted_by(self.get_cell(x, y + 1), owner, root_coord) {
            return coord::new(x, y + 1);
        }
        panic!(
            "\x1b[31mNo adjacent organ found for coord x: {:?} y: {:?}\x1b[0m",
            coord::x(coord),
            coord::y(coord)
        );
    }

    pub fn add_organ(&mut self, coord: Coord, organ: Organ) {
        let x = coord::x(coord);
        let y = coord::y(coord);
        self.set_cell(x, y, cell::new(false, None, Some(organ)));

        if OrganType::Root == get_type(organ) {
            return;
        }

        let parent_cell = self.get_one_adjacent_organ(
            coord,
            organ::get_owner(organ),
            organ::get_root_coord(organ),
        );

        let connections = self
            .cell_connections
            .entry(parent_cell)
            .or_insert(HashSet::new());
        connections.insert(coord);
    }

    pub fn remove_organ(&mut self, coord: Coord) {
        let x = coord::x(coord);
        let y = coord::y(coord);
        let cell = self.get_cell(x, y);
        if let Some(organ) = cell::get_organ(cell) {
            if OrganType::Root != get_type(organ) {
                let parent_cell = self.get_one_adjacent_organ(
                    coord,
                    organ::get_owner(organ),
                    organ::get_root_coord(organ),
                );
                let connections = self.cell_connections.get_mut(&parent_cell).unwrap();
                connections.retain(|&c| c != coord);
            }
        }
        self.remove_children(coord);
    }

    fn remove_children(&mut self, coord: Coord) {
        if !cell::is_organ(self.get_cell_from_coord(coord)) {
            panic!("\x1b[31Cell is not an organ\x1b[0m");
        }
        self.set_cell(
            coord::x(coord),
            coord::y(coord),
            cell::new(false, None, None),
        );
        if let Some(children) = self.cell_connections.remove(&coord) {
            children.iter().for_each(|child_coord| {
                self.remove_children(*child_coord);
            });
        }
    }

    pub fn can_add_organ(&self, dest: Coord, organ: Organ) -> bool {
        let x = coord::x(dest);
        let y = coord::y(dest);
        (self.is_in_bounds(x, y)
            && cell::is_empty(self.get_cell(x, y))
            && !self.is_canceled_by_tentacle(x, y, organ::get_owner(organ)))
            && (OrganType::Root == get_type(organ)
                || self.contains_an_adjacent_organ(x, y, organ::get_owner(organ)))
    }

    fn is_canceled_by_tentacle(&self, x: u8, y: u8, owner: u8) -> bool {
        let initial_coord = coord::new(x, y);
        (x > 0 && self.is_front_of_tentacle(initial_coord, coord::new(x + 1, y), owner))
            || (self.is_front_of_tentacle(initial_coord, coord::new(x + 1, y), owner))
            || (y > 0 && self.is_front_of_tentacle(initial_coord, coord::new(x, y - 1), owner))
            || (self.is_front_of_tentacle(initial_coord, coord::new(x, y + 1), owner))
    }

    fn is_front_of_tentacle(&self, initial_coord: Coord, tentacle_coord: Coord, owner: u8) -> bool {
        let tentacle_cell = self.get_cell_from_coord(tentacle_coord);
        cell::is_tentacle(tentacle_cell)
            && cell::is_owned_by(tentacle_cell, 1 - owner)
            && organ::is_faced_to(
                cell::get_organ(tentacle_cell).unwrap(),
                tentacle_coord,
                initial_coord,
            )
    }

    fn contains_an_adjacent_organ(&self, x: u8, y: u8, owner: u8) -> bool {
        (x > 0 && cell::is_owned_by(self.get_cell(x - 1, y), owner))
            || cell::is_owned_by(self.get_cell(x + 1, y), owner)
            || (y > 0 && cell::is_owned_by(self.get_cell(x, y - 1), owner))
            || cell::is_owned_by(self.get_cell(x, y + 1), owner)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_entities::{cell, organ_direction::OrganDirection, protein::Protein};

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

    #[test]
    fn test_get_adjacent_coords() {
        let grid = Grid::new(3, 3);

        assert_eq!(
            grid.get_adjacent_coords(coord::new(1, 1)),
            vec![
                coord::new(0, 1),
                coord::new(2, 1),
                coord::new(1, 0),
                coord::new(1, 2)
            ]
        );
    }

    #[test]
    fn test_get_adjacent_cells() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 1, cell::new(true, None, None));
        grid.set_cell(1, 0, cell::new(true, None, None));
        grid.set_cell(1, 2, cell::new(true, None, None));
        grid.set_cell(2, 1, cell::new(true, None, None));

        assert_eq!(
            grid.get_adjacent_cells(coord::new(1, 1)),
            vec![
                cell::new(true, None, None),
                cell::new(true, None, None),
                cell::new(true, None, None),
                cell::new(true, None, None)
            ]
        );
    }

    #[test]
    fn test_get_adjacent_cells_out_of_bounds() {
        let grid = Grid::new(3, 3);

        assert_eq!(
            grid.get_adjacent_cells(coord::new(0, 0)),
            vec![cell::new(false, None, None), cell::new(false, None, None)]
        );
    }

    #[test]
    fn test_can_add_organ() {
        let mut grid = Grid::new(6, 6);

        grid.set_cell(0, 0, cell::new(false, None, None));
        grid.set_cell(1, 0, cell::new(false, None, None));
        grid.set_cell(2, 0, cell::new(false, None, None));
        grid.set_cell(0, 1, cell::new(false, None, None));
        grid.set_cell(1, 1, cell::new(false, None, None));
        grid.set_cell(2, 1, cell::new(false, None, None));
        grid.set_cell(0, 2, cell::new(false, None, None));
        grid.set_cell(1, 2, cell::new(false, None, None));
        grid.set_cell(2, 2, cell::new(false, None, None));

        let default_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);

        assert_eq!(grid.can_add_organ(coord::new(0, 0), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(1, 0), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(2, 0), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(0, 1), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(1, 1), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(2, 1), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(0, 2), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(1, 2), default_organ), true);
        assert_eq!(grid.can_add_organ(coord::new(2, 2), default_organ), true);
    }

    #[test]
    fn test_can_add_organ_out_of_bounds() {
        let grid = Grid::new(3, 3);

        let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);

        assert_eq!(grid.can_add_organ(coord::new(3, 3), default_organ), false);
    }

    #[test]
    fn test_can_add_organ_with_obstacle() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(true, None, None));

        let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);

        assert_eq!(grid.can_add_organ(coord::new(0, 0), default_organ), false);
    }

    #[test]
    fn test_can_add_organ_without_adjacent_organ() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(0, 0, cell::new(false, Some(Protein::A), None));

        let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);

        assert_eq!(grid.can_add_organ(coord::new(1, 0), default_organ), false);
    }

    #[test]
    fn test_can_add_organ_with_adjacent_organ() {
        let mut grid = Grid::new(3, 3);
        let default_organ = organ::new(1, OrganType::Basic, OrganDirection::North, 0);

        grid.set_cell(0, 0, cell::new(false, None, Some(default_organ)));

        assert_eq!(grid.can_add_organ(coord::new(1, 0), default_organ), true);
    }

    #[test]
    fn test_can_add_organ_with_adjacent_not_owned_organ() {
        let mut grid = Grid::new(3, 3);
        let default_organ0 = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
        let default_organ1 = organ::new(1, OrganType::Basic, OrganDirection::North, 0);

        grid.set_cell(0, 0, cell::new(false, None, Some(default_organ0)));

        assert_eq!(grid.can_add_organ(coord::new(3, 3), default_organ1), false);
    }

    #[test]
    fn test_can_add_organ_in_front_of_tentacle() {
        let mut grid = Grid::new(5, 5);
        let root_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);
        let tentacle_organ = organ::new(0, OrganType::Tentacle, OrganDirection::South, 0);
        let root_organ1 = organ::new(1, OrganType::Root, OrganDirection::South, 0);

        grid.set_cell(0, 0, cell::new(false, None, Some(root_organ)));
        grid.set_cell(0, 1, cell::new(false, None, Some(tentacle_organ)));

        assert_eq!(grid.can_add_organ(coord::new(0, 2), root_organ1), false);
        assert_eq!(grid.can_add_organ(coord::new(0, 2), root_organ), true);
    }

    #[test]
    fn test_add_organ() {
        let mut grid = Grid::new(3, 3);
        let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
        let root_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);

        grid.add_organ(coord::new(0, 0), root_organ);
        grid.add_organ(coord::new(1, 0), default_organ);

        assert_eq!(cell::get_organ(grid.get_cell(1, 0)).unwrap(), default_organ);

        let connections = grid.cell_connections.get(&coord::new(0, 0)).unwrap();
        assert_eq!(connections.len(), 1);

        grid.add_organ(coord::new(0, 1), default_organ);
        grid.add_organ(coord::new(1, 1), default_organ);

        let connections = grid.cell_connections.get(&coord::new(1, 1));
        assert_eq!(connections.is_none(), true);

        let connections = grid.cell_connections.get(&coord::new(0, 0)).unwrap();
        assert_eq!(connections.len(), 2);
    }

    #[test]
    pub fn test_remove_organ() {
        let mut grid = Grid::new(3, 3);
        let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
        let root_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);

        grid.add_organ(coord::new(0, 0), root_organ);
        grid.add_organ(coord::new(1, 0), default_organ);
        grid.add_organ(coord::new(0, 1), default_organ);
        grid.add_organ(coord::new(1, 1), default_organ);

        grid.remove_organ(coord::new(1, 0));

        assert_eq!(cell::is_empty(grid.get_cell(1, 0)), true);

        let connections = grid.cell_connections.get(&coord::new(0, 0)).unwrap();
        assert_eq!(connections.len(), 1);

        grid.remove_organ(coord::new(0, 0));

        assert_eq!(grid.cells.iter().all(|&cell| cell::is_empty(cell)), true);

        let connections = grid.cell_connections.get(&coord::new(0, 0));
        assert_eq!(connections.is_none(), true);
    }

    #[test]
    pub fn test_remove_organ_no_child() {
        let mut grid = Grid::new(8, 8);
        let root_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);

        grid.add_organ(coord::new(0, 0), root_organ);
        grid.add_organ(coord::new(1, 0), root_organ);
        grid.add_organ(coord::new(0, 1), root_organ);
        grid.add_organ(coord::new(1, 1), root_organ);
        grid.add_organ(coord::new(1, 2), root_organ);
        grid.add_organ(coord::new(2, 1), root_organ);

        grid.remove_organ(coord::new(1, 1));

        assert_eq!(cell::is_empty(grid.get_cell(0, 0)), false);
        assert_eq!(cell::is_empty(grid.get_cell(0, 1)), false);
        assert_eq!(cell::is_empty(grid.get_cell(1, 1)), true);
        assert_eq!(cell::is_empty(grid.get_cell(1, 0)), false);
        assert_eq!(cell::is_empty(grid.get_cell(2, 1)), false);
        assert_eq!(cell::is_empty(grid.get_cell(1, 2)), false);

        let connections = grid.cell_connections.get(&coord::new(0, 0));
        assert_eq!(connections.is_none(), true);
    }
}
