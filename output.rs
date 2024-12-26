use std::io;
pub mod actions {
    pub mod action {
        use super::action_type::{self, ActionType};
        use crate::game_entities::{
            coord::Coord, grid::Grid, organ, organ_direction::OrganDirection,
            organ_type::OrganType, player::Player, protein_wallet,
        };
        const MASK_TYPE: Action = 0b0000_0011;
        const MASK_ORGAN: Action = 0b0001_1100;
        const MASK_DIRECTION: Action = 0b0110_0000;
        const MASK_COORD_TARGET: Action = 0b0000_0000_1111_1111_1000_0000;
        const MASK_COORD_ROOT: Action = 0x01_FF_00_00;
        pub type Action = u32;
        pub fn new(
            action_type: ActionType,
            organ_type: OrganType,
            direction: OrganDirection,
            coord_target: Coord,
            coord_root: Coord,
        ) -> Action {
            let action_type = action_type as Action;
            let organ_type = (organ_type as Action) << 2;
            let direction = (direction as Action) << 5;
            let coord_target = (coord_target as Action) << 7;
            let coord_source = (coord_root as Action) << 16;
            action_type | organ_type | direction | coord_target | coord_source
        }
        pub fn get_type(action: Action) -> ActionType {
            action_type::ActionType::from_index((action & MASK_TYPE) as usize)
        }
        pub fn get_organ_type(action: Action) -> OrganType {
            OrganType::from_index(((action & MASK_ORGAN) >> 2) as usize)
        }
        pub fn get_direction(action: Action) -> OrganDirection {
            OrganDirection::from_index(((action & MASK_DIRECTION) >> 5) as usize)
        }
        pub fn get_coord_target(action: Action) -> Coord {
            ((action & MASK_COORD_TARGET) >> 7) as Coord
        }
        pub fn get_coord_source(action: Action) -> Coord {
            ((action & MASK_COORD_ROOT) >> 16) as Coord
        }
        pub fn set_coord_target(action: Action, coord: Coord) -> Action {
            (action & !MASK_COORD_TARGET) | ((coord as Action) << 7)
        }
        pub fn set_coord_source(action: Action, coord: Coord) -> Action {
            (action & !MASK_COORD_ROOT) | ((coord as Action) << 16)
        }
        pub fn set_direction(action: Action, direction: OrganDirection) -> Action {
            (action & !MASK_DIRECTION) | ((direction as Action) << 5)
        }
        pub fn set_organ_type(action: Action, organ_type: OrganType) -> Action {
            (action & !MASK_ORGAN) | ((organ_type as Action) << 2)
        }
        pub fn wait() -> Action {
            new(
                ActionType::Wait,
                OrganType::Root,
                OrganDirection::North,
                0,
                0,
            )
        }
        pub fn growth(
            organ_type: OrganType,
            direction: OrganDirection,
            coord_target: Coord,
            coord_root: Coord,
        ) -> Action {
            new(
                ActionType::Growth,
                organ_type,
                direction,
                coord_target,
                coord_root,
            )
        }
        pub fn sporer(direction: OrganDirection, coord_target: Coord, coord_root: Coord) -> Action {
            new(
                ActionType::Sporer,
                OrganType::Root,
                direction,
                coord_target,
                coord_root,
            )
        }
        pub fn is_valid(action: Action, grid: &Grid, player: &Player) -> bool {
            let action_type = get_type(action);
            let coord_target = get_coord_target(action);
            let coord_root = get_coord_source(action);
            let direction = get_direction(action);
            if ActionType::Wait == action_type {
                return true;
            }
            let organ_type = if ActionType::Growth == action_type {
                get_organ_type(action)
            } else {
                OrganType::Root
            };
            protein_wallet::can_buy_organ(player.get_wallet(), organ_type)
                && grid.can_add_organ_without_root_coord(
                    coord_target,
                    organ::new(player.get_id(), organ_type, direction, coord_root),
                )
        }
        #[cfg(test)]
        mod tests {
            use super::action_type::ActionType;
            use super::*;
            use crate::game_entities::organ_direction::OrganDirection;
            use crate::game_entities::organ_type::OrganType;
            use crate::game_entities::protein::Protein;
            use crate::game_entities::{cell, coord};
            #[test]
            fn test_action() {
                let action = new(
                    ActionType::Growth,
                    OrganType::Root,
                    OrganDirection::North,
                    coord::new(1, 1),
                    coord::new(2, 2),
                );
                assert_eq!(get_type(action), ActionType::Growth);
                assert_eq!(get_organ_type(action), OrganType::Root);
                assert_eq!(get_direction(action), OrganDirection::North);
                assert_eq!(get_coord_target(action), coord::new(1, 1));
                assert_eq!(get_coord_source(action), coord::new(2, 2));
            }
            #[test]
            fn test_action_with_max() {
                let action = new(
                    ActionType::Growth,
                    OrganType::Root,
                    OrganDirection::North,
                    coord::new(26, 13),
                    coord::new(26, 13),
                );
                assert_eq!(get_type(action), ActionType::Growth);
                assert_eq!(get_organ_type(action), OrganType::Root);
                assert_eq!(get_direction(action), OrganDirection::North);
                assert_eq!(get_coord_target(action), coord::new(26, 13));
                assert_eq!(get_coord_source(action), coord::new(26, 13));
            }
            #[test]
            fn test_is_valid() {
                let mut player = Player::new(0);
                player.add_protein(Protein::A, 1);
                player.add_protein(Protein::B, 1);
                player.add_protein(Protein::C, 1);
                player.add_protein(Protein::D, 1);
                let grid = Grid::new(10, 10);
                let action = new(
                    ActionType::Growth,
                    OrganType::Root,
                    OrganDirection::North,
                    coord::new(1, 1),
                    coord::new(2, 2),
                );
                assert!(is_valid(action, &grid, &player));
            }
            #[test]
            fn test_is_not_valid() {
                let mut player = Player::new(0);
                let mut grid = Grid::new(10, 10);
                let action = new(
                    ActionType::Growth,
                    OrganType::Root,
                    OrganDirection::North,
                    coord::new(1, 1),
                    coord::new(2, 2),
                );
                assert!(!is_valid(action, &grid, &player));
                player.add_protein(Protein::A, 1);
                assert!(!is_valid(action, &grid, &player));
                player.add_protein(Protein::B, 1);
                assert!(!is_valid(action, &grid, &player));
                player.add_protein(Protein::C, 1);
                assert!(!is_valid(action, &grid, &player));
                player.add_protein(Protein::D, 1);
                assert!(is_valid(action, &grid, &player));
                grid.set_cell(
                    2,
                    2,
                    cell::new(
                        false,
                        None,
                        Some(organ::new(
                            0,
                            OrganType::Root,
                            OrganDirection::North,
                            coord::new(2, 2),
                        )),
                    ),
                );
                let action = new(
                    ActionType::Growth,
                    OrganType::Basic,
                    OrganDirection::North,
                    coord::new(2, 3),
                    coord::new(2, 2),
                );
                assert!(is_valid(action, &grid, &player));
                let action = new(
                    ActionType::Growth,
                    OrganType::Basic,
                    OrganDirection::North,
                    coord::new(2, 2),
                    coord::new(2, 2),
                );
                assert!(!is_valid(action, &grid, &player));
                let action = new(
                    ActionType::Growth,
                    OrganType::Basic,
                    OrganDirection::North,
                    coord::new(2, 2),
                    coord::new(2, 3),
                );
                assert!(!is_valid(action, &grid, &player));
                let action = new(
                    ActionType::Growth,
                    OrganType::Basic,
                    OrganDirection::North,
                    coord::new(2, 4),
                    coord::new(2, 2),
                );
                assert!(!is_valid(action, &grid, &player));
            }
            #[test]
            fn test_is_valid_wait() {
                let player = Player::new(0);
                let grid = Grid::new(10, 10);
                let action = new(
                    ActionType::Wait,
                    OrganType::Root,
                    OrganDirection::North,
                    coord::new(1, 1),
                    coord::new(2, 2),
                );
                assert!(is_valid(action, &grid, &player));
            }
        }
    }
    pub mod action_type {
        #[derive(Debug, PartialEq)]
        pub enum ActionType {
            Wait = 0b00,
            Growth = 0b01,
            Sporer = 0b10,
        }
        impl ActionType {
            pub fn from_str(s: &str) -> ActionType {
                match s {
                    "WAIT" => ActionType::Wait,
                    "GROWTH" => ActionType::Growth,
                    "SPORER" => ActionType::Sporer,
                    _ => panic!("Invalid action type {}", s),
                }
            }
            pub fn from_index(i: usize) -> ActionType {
                match i {
                    0b00 => ActionType::Wait,
                    0b01 => ActionType::Growth,
                    0b10 => ActionType::Sporer,
                    _ => panic!("Invalid action type index {}", i),
                }
            }
            pub fn to_str(&self) -> &str {
                match self {
                    ActionType::Wait => "WAIT",
                    ActionType::Growth => "GROWTH",
                    ActionType::Sporer => "SPORER",
                }
            }
            pub fn get_index(&self) -> u8 {
                match self {
                    ActionType::Wait => 0,
                    ActionType::Growth => 1,
                    ActionType::Sporer => 2,
                }
            }
        }
    }
    pub mod action_validator {
        use super::{action, action_type::ActionType};
        use crate::game_entities::{
            coord,
            grid::Grid,
            organ,
            organ_type::{self, OrganType},
        };
        pub fn make_it_valid(action: action::Action, grid: &Grid) -> action::Action {
            match action::get_type(action) {
                ActionType::Growth => make_growth_valid(action, grid),
                ActionType::Sporer => action::wait(),
                ActionType::Wait => action::wait(),
            }
        }
        pub fn make_growth_valid(action: action::Action, grid: &Grid) -> action::Action {
            let x_coord = coord::x(action::get_coord_target(action));
            let y_coord = coord::y(action::get_coord_target(action));
            let mut result = action;
            if action::get_organ_type(action) == OrganType::Root {
                let organ_type = (rand::random::<usize>()) % 0b100;
                result = action::set_organ_type(result, OrganType::from_index(organ_type));
            }
            let organ = organ::new(
                0,
                action::get_organ_type(result),
                action::get_direction(result),
                action::get_coord_source(result),
            );
            for x in 0..grid.width as usize {
                if grid.can_add_organ_with_root_coord(coord::new(x as u8, y_coord), organ) {
                    return action::set_coord_target(result, coord::new(x as u8, y_coord));
                }
            }
            for y in 0..grid.height as usize {
                if grid.can_add_organ_with_root_coord(coord::new((x_coord) as u8, y as u8), organ) {
                    return action::set_coord_target(result, coord::new(x_coord, y as u8));
                }
            }
            action::wait()
        }
        fn make_sporer_valid(
            previous_action: action::Action,
            action: action::Action,
            grid: &Grid,
        ) -> action::Action {
            let x_coord = coord::x(action::get_coord_target(action));
            let y_coord = coord::y(action::get_coord_target(action));
            let organ = organ::new(
                0,
                action::get_organ_type(action),
                action::get_direction(action),
                action::get_coord_source(action),
            );
            for x in 0..grid.width as usize {
                if (x > 0
                    && grid.can_add_organ_without_root_coord(
                        coord::new((x - 1) as u8, y_coord),
                        organ,
                    ))
                    || (x < (grid.width as usize))
                        && grid.can_add_organ_without_root_coord(
                            coord::new((x + 1) as u8, y_coord),
                            organ,
                        )
                {
                    return action::set_coord_target(action, coord::new(x as u8, y_coord));
                }
            }
            for y in 0..grid.height as usize {
                if (y > 0
                    && grid.can_add_organ_without_root_coord(
                        coord::new((x_coord) as u8, (y - 1) as u8),
                        organ,
                    ))
                    || (y < (grid.height as usize))
                        && grid.can_add_organ_without_root_coord(
                            coord::new((x_coord) as u8, (y + 1) as u8),
                            organ,
                        )
                {
                    return action::set_coord_target(action, coord::new(x_coord, y as u8));
                }
            }
            action::wait()
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_entities::organ_type::OrganType;
            #[test]
            fn test_make_it_valid() {
                let grid = Grid::new(5, 5);
                let action = action::growth(
                    OrganType::Basic,
                    action::get_direction(action::wait()),
                    coord::new(0, 0),
                    coord::new(0, 0),
                );
                let valid_action = make_it_valid(action, &grid);
                assert_eq!(action::get_type(valid_action), ActionType::Wait);
            }
            #[test]
            fn test_make_growth_valid() {
                let grid = Grid::new(5, 5);
                let action = action::growth(
                    OrganType::Root,
                    action::get_direction(action::wait()),
                    coord::new(0, 0),
                    coord::new(0, 0),
                );
                let valid_action = make_growth_valid(action, &grid);
                assert_eq!(action::get_type(valid_action), ActionType::Wait);
            }
            #[test]
            fn test_make_sporer_valid() {
                let grid = Grid::new(5, 5);
                let action = action::sporer(
                    action::get_direction(action::wait()),
                    coord::new(0, 0),
                    coord::new(0, 0),
                );
                let valid_action = make_sporer_valid(action::wait(), action, &grid);
                assert_eq!(action::get_type(valid_action), ActionType::Wait);
            }
        }
    }
    pub mod actions_finder {}
}
pub mod game_entities {
    pub mod protein_wallet {
        use super::{organ_type::OrganType, protein::Protein};
        pub type ProteinWallet = u32;
        const MASK_PROTEIN: ProteinWallet = 0xff;
        const BITS_PROTEIN: u32 = 8;
        pub fn new() -> ProteinWallet {
            0
        }
        pub fn add(wallet: &mut ProteinWallet, protein_type: Protein, amount: u32) {
            *wallet += amount << (protein_type as ProteinWallet * BITS_PROTEIN);
        }
        pub fn remove(wallet: &mut ProteinWallet, protein_type: Protein, amount: u32) {
            *wallet -= amount << (protein_type as ProteinWallet * BITS_PROTEIN);
        }
        pub fn get(wallet: ProteinWallet, protein_type: Protein) -> u8 {
            ((wallet >> (protein_type as ProteinWallet * BITS_PROTEIN)) & MASK_PROTEIN) as u8
        }
        pub fn can_buy_organ(wallet: ProteinWallet, organ_type: OrganType) -> bool {
            let cost = organ_type.get_cost();
            get(wallet, Protein::A) >= get(cost, Protein::A)
                && get(wallet, Protein::B) >= get(cost, Protein::B)
                && get(wallet, Protein::C) >= get(cost, Protein::C)
                && get(wallet, Protein::D) >= get(cost, Protein::D)
        }
        pub fn buy_organ(wallet: &mut ProteinWallet, organ_type: OrganType) {
            let cost = organ_type.get_cost();
            remove(wallet, Protein::A, get(cost, Protein::A) as ProteinWallet);
            remove(wallet, Protein::B, get(cost, Protein::B) as ProteinWallet);
            remove(wallet, Protein::C, get(cost, Protein::C) as ProteinWallet);
            remove(wallet, Protein::D, get(cost, Protein::D) as ProteinWallet);
        }
    }
    pub mod cell {
        use super::{
            coord::Coord,
            organ::{self, Organ},
            protein::Protein,
        };
        pub type Cell = u16;
        const EMPTY: Cell = 0b00;
        const OBSTACLE: Cell = 0b01;
        const PROTEIN: Cell = 0b10;
        const ORGAN: Cell = 0b11;
        const MASK_TYPE: Cell = 0b11;
        pub fn new(obstacle: bool, protein: Option<Protein>, organ: Option<Organ>) -> Cell {
            match (obstacle, protein, organ) {
                (true, _, _) => OBSTACLE,
                (false, None, None) => EMPTY,
                (false, Some(protein), None) => (protein as u16) << 2 | PROTEIN,
                (false, None, Some(organ)) => ((organ as u16) << 2) | ORGAN,
                _ => panic!("Invalid cell"),
            }
        }
        pub fn new_wall() -> Cell {
            OBSTACLE
        }
        pub fn get_protein(cell: Cell) -> Option<Protein> {
            if cell & 0b11 == PROTEIN {
                Protein::from_id((cell >> 2) as u8)
            } else {
                None
            }
        }
        pub fn get_organ(cell: Cell) -> Option<Organ> {
            if contains_organ(cell) {
                Some((cell >> 2) as Organ)
            } else {
                panic!("\x1b[31mCell {:?} does not contain an organ\x1b[0m", cell);
            }
        }
        pub fn set_protein(cell: &mut Cell, protein: Protein) {
            *cell = (protein as u16) << 2 | PROTEIN;
        }
        pub fn set_obstacle(cell: &mut Cell) {
            *cell = OBSTACLE;
        }
        pub fn unset_obstacle(cell: &mut Cell) {
            *cell = EMPTY;
        }
        pub fn place_organ(cell: &mut Cell, organ_id: u16) {
            *cell = organ_id << 2 | ORGAN;
        }
        pub fn is_obstacle(cell: Cell) -> bool {
            get_type_cell(cell) == OBSTACLE
        }
        pub fn is_empty(cell: Cell) -> bool {
            get_type_cell(cell) == EMPTY
        }
        pub fn is_protein(cell: Cell) -> bool {
            get_type_cell(cell) == PROTEIN
        }
        pub fn is_organ(cell: Cell) -> bool {
            get_type_cell(cell) == ORGAN
        }
        pub fn is_tentacle(cell: Cell) -> bool {
            is_organ(cell) && organ::is_tentacle(get_organ(cell).unwrap())
        }
        pub fn is_owned_by(cell: Cell, owner: u8) -> bool {
            is_organ(cell) && organ::get_owner(get_organ(cell).unwrap()) == owner
        }
        pub fn is_owned_and_rooted_by(cell: Cell, owner: u8, root_coord: Coord) -> bool {
            is_owned_by(cell, owner)
                && organ::get_root_coord(get_organ(cell).unwrap()) == root_coord
        }
        pub fn has_root_coord(cell1: Cell, root_coord: Coord) -> bool {
            is_organ(cell1) && organ::get_root_coord(get_organ(cell1).unwrap()) == root_coord
        }
        pub fn contains_organ(cell: Cell) -> bool {
            get_type_cell(cell) == ORGAN
        }
        pub fn contains_protein(cell: Cell) -> bool {
            get_type_cell(cell) == PROTEIN
        }
        fn get_type_cell(cell: Cell) -> Cell {
            cell & MASK_TYPE
        }
        pub fn clear(cell: &mut Cell) {
            *cell = EMPTY;
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_entities::cell;
            #[test]
            fn test_cell() {
                let cell = cell::new(false, Some(Protein::A), None);
                assert_eq!(cell::is_obstacle(cell), false);
                assert_eq!(cell::get_protein(cell), Some(Protein::A));
                assert_eq!(cell::contains_organ(cell), false);
                assert_eq!(cell::contains_protein(cell), true);
            }
            #[test]
            fn test_cell_wall() {
                let cell = cell::new(true, None, None);
                assert_eq!(cell::is_obstacle(cell), true);
                assert_eq!(cell::get_protein(cell), None);
                assert_eq!(cell::contains_organ(cell), false);
                assert_eq!(cell::contains_protein(cell), false);
            }
            #[test]
            fn test_cell_organ() {
                let cell = cell::new(false, None, Some(1));
                assert_eq!(cell::is_obstacle(cell), false);
                assert_eq!(cell::get_protein(cell), None);
                assert_eq!(cell::contains_organ(cell), true);
                assert_eq!(cell::contains_protein(cell), false);
            }
        }
    }
    pub mod coord {
        pub type Coord = u16;
        const BITS_Y: Coord = 4;
        const MASK_Y: Coord = 0b1111;
        const MASK_X: Coord = 0b1_1111;
        pub const MASK_COORD: Coord = 0b0000_0001_1111_1111;
        const MAX_X: u8 = 26;
        const MAX_Y: u8 = 13;
        pub fn new(x: u8, y: u8) -> Coord {
            if x > MAX_X || y > MAX_Y {
                panic!("\x1b[31mInvalid coordinates ({}, {})\x1b[0m", x, y);
            }
            ((x as Coord) << BITS_Y) | y as Coord
        }
        pub fn x(coord: Coord) -> u8 {
            ((coord >> BITS_Y) & MASK_X) as u8
        }
        pub fn y(coord: Coord) -> u8 {
            (coord & MASK_Y) as u8
        }
        pub fn manhattan_distance(coord1: Coord, coord2: Coord) -> u8 {
            x(coord1).abs_diff(x(coord2)) + y(coord1).abs_diff(y(coord2))
        }
        pub fn is_adjacent(coord1: Coord, coord2: Coord) -> bool {
            manhattan_distance(coord1, coord2) == 1
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            #[should_panic]
            fn test_coord_new_invalid_x() {
                new(27, 0);
            }
            #[test]
            #[should_panic]
            fn test_coord_new_invalid_y() {
                new(0, 14);
            }
            #[test]
            fn test_pos_new() {
                let pos = new(26, 13);
                assert_eq!(x(pos), 26);
                assert_eq!(y(pos), 13);
            }
            #[test]
            fn test_pos_manhattan_distance() {
                let pos1 = new(1, 1);
                let pos2 = new(4, 5);
                assert_eq!(manhattan_distance(pos1, pos2), 7);
            }
            #[test]
            fn test_pos_is_adjacent() {
                let pos1 = new(1, 1);
                let pos2 = new(1, 2);
                assert!(is_adjacent(pos1, pos2));
            }
            #[test]
            fn test_pos_is_not_adjacent() {
                let pos1 = new(1, 1);
                let pos2 = new(1, 3);
                assert!(!is_adjacent(pos1, pos2));
            }
            #[test]
            fn test_partial_eq() {
                let pos1 = new(1, 1);
                let pos2 = new(1, 1);
                assert_eq!(pos1, pos2);
            }
            #[test]
            fn test_partial_not_eq() {
                let pos1 = new(1, 1);
                let pos2 = new(1, 2);
                assert_ne!(pos1, pos2);
            }
            #[test]
            fn test_clone() {
                let pos1 = new(1, 1);
                let pos2 = pos1.clone();
                assert_eq!(pos1, pos2);
            }
        }
    }
    pub mod grid {
        use super::{
            cell::{self, Cell},
            coord::{self, Coord},
            organ::{self, get_type, Organ},
            organ_type::OrganType,
        };
        use std::collections::{HashMap, HashSet};
        pub const MAX_WIDTH: usize = 24;
        pub const MAX_HEIGHT: usize = 12;
        pub struct Grid {
            cells: [Cell; MAX_WIDTH * MAX_HEIGHT],
            cell_connections: HashMap<coord::Coord, HashSet<coord::Coord>>,
            pub width: u8,
            pub height: u8,
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
                if x > 0 && cell::is_owned_and_rooted_by(self.get_cell(x - 1, y), owner, root_coord)
                {
                    return coord::new(x - 1, y);
                }
                if cell::is_owned_and_rooted_by(self.get_cell(x + 1, y), owner, root_coord) {
                    return coord::new(x + 1, y);
                }
                if y > 0 && cell::is_owned_and_rooted_by(self.get_cell(x, y - 1), owner, root_coord)
                {
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
            pub fn can_add_organ_without_root_coord(&self, dest: Coord, organ: Organ) -> bool {
                let x = coord::x(dest);
                let y = coord::y(dest);
                (self.is_in_bounds(x, y)
                    && cell::is_empty(self.get_cell(x, y))
                    && !self.is_canceled_by_tentacle(x, y, organ::get_owner(organ)))
                    && (OrganType::Root == get_type(organ)
                        || self.contains_an_adjacent_organ(x, y, organ::get_owner(organ)))
            }
            pub fn can_add_organ_with_root_coord(&self, dest: Coord, organ: Organ) -> bool {
                let x = coord::x(dest);
                let y = coord::y(dest);
                (self.is_in_bounds(x, y)
                    && cell::is_empty(self.get_cell(x, y))
                    && !self.is_canceled_by_tentacle(x, y, organ::get_owner(organ)))
                    && (OrganType::Root == get_type(organ)
                        || self.contains_an_adjacent_organ_with_same_root(
                            x,
                            y,
                            organ::get_root_coord(organ),
                        ))
            }
            fn is_canceled_by_tentacle(&self, x: u8, y: u8, owner: u8) -> bool {
                let initial_coord = coord::new(x, y);
                (x > 0 && self.is_front_of_tentacle(initial_coord, coord::new(x - 1, y), owner))
                    || (x < self.width - 1
                        && self.is_front_of_tentacle(initial_coord, coord::new(x + 1, y), owner))
                    || (y > 0
                        && self.is_front_of_tentacle(initial_coord, coord::new(x, y - 1), owner))
                    || (y < self.height - 1
                        && self.is_front_of_tentacle(initial_coord, coord::new(x, y + 1), owner))
            }
            fn is_front_of_tentacle(
                &self,
                initial_coord: Coord,
                tentacle_coord: Coord,
                owner: u8,
            ) -> bool {
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
            fn contains_an_adjacent_organ_with_same_root(
                &self,
                x: u8,
                y: u8,
                root_coord: Coord,
            ) -> bool {
                (x > 0 && cell::has_root_coord(self.get_cell(x - 1, y), root_coord))
                    || (x < self.width - 1
                        && cell::has_root_coord(self.get_cell(x + 1, y), root_coord))
                    || (y > 0 && cell::has_root_coord(self.get_cell(x, y - 1), root_coord))
                    || (y < self.height - 1
                        && cell::has_root_coord(self.get_cell(x, y + 1), root_coord))
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_entities::{cell, organ_direction::OrganDirection, protein::Protein};
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
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 0), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(1, 0), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(2, 0), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 1), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(1, 1), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(2, 1), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 2), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(1, 2), default_organ),
                    true
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(2, 2), default_organ),
                    true
                );
            }
            #[test]
            fn test_can_add_organ_out_of_bounds() {
                let grid = Grid::new(3, 3);
                let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(3, 3), default_organ),
                    false
                );
            }
            #[test]
            fn test_can_add_organ_in_edge() {
                let grid = Grid::new(3, 3);
                let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(2, 2), default_organ),
                    true
                );
            }
            #[test]
            fn test_can_add_organ_with_obstacle() {
                let mut grid = Grid::new(3, 3);
                grid.set_cell(0, 0, cell::new(true, None, None));
                let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 0), default_organ),
                    false
                );
            }
            #[test]
            fn test_can_add_organ_without_adjacent_organ() {
                let mut grid = Grid::new(3, 3);
                grid.set_cell(0, 0, cell::new(false, Some(Protein::A), None));
                let default_organ = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(1, 0), default_organ),
                    false
                );
            }
            #[test]
            fn test_can_add_organ_with_adjacent_organ() {
                let mut grid = Grid::new(3, 3);
                let default_organ = organ::new(1, OrganType::Basic, OrganDirection::North, 0);
                grid.set_cell(0, 0, cell::new(false, None, Some(default_organ)));
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(1, 0), default_organ),
                    true
                );
            }
            #[test]
            fn test_can_add_organ_with_adjacent_not_owned_organ() {
                let mut grid = Grid::new(3, 3);
                let default_organ0 = organ::new(0, OrganType::Basic, OrganDirection::North, 0);
                let default_organ1 = organ::new(1, OrganType::Basic, OrganDirection::North, 0);
                grid.set_cell(0, 0, cell::new(false, None, Some(default_organ0)));
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(3, 3), default_organ1),
                    false
                );
            }
            #[test]
            fn test_can_add_organ_in_front_of_tentacle() {
                let mut grid = Grid::new(5, 5);
                let root_organ = organ::new(0, OrganType::Root, OrganDirection::North, 0);
                let tentacle_organ = organ::new(0, OrganType::Tentacle, OrganDirection::South, 0);
                let root_organ1 = organ::new(1, OrganType::Root, OrganDirection::South, 0);
                grid.set_cell(0, 0, cell::new(false, None, Some(root_organ)));
                grid.set_cell(0, 1, cell::new(false, None, Some(tentacle_organ)));
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 2), root_organ1),
                    false
                );
                assert_eq!(
                    grid.can_add_organ_without_root_coord(coord::new(0, 2), root_organ),
                    true
                );
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
    }
    pub mod organ {
        use super::{coord::Coord, organ_direction::OrganDirection, organ_type::OrganType};
        const MASK_PLAYER: Organ = 0b0000_0001;
        const MASK_ORGAN_TYPE: Organ = 0b0001_1110;
        const MASK_ORGAN_DIRECTION: Organ = 0b1110_0000;
        const MASK_ROOT_ID: Organ = 0xFF_FF00;
        pub type Organ = u32;
        pub fn new(
            owner: u8,
            organ_type: OrganType,
            organ_direction: OrganDirection,
            root_coord: Coord,
        ) -> Organ {
            (owner as Organ & MASK_PLAYER)
                | ((organ_type as Organ) << 1 & MASK_ORGAN_TYPE)
                | ((organ_direction as Organ) << 5 & MASK_ORGAN_DIRECTION)
                | ((root_coord as Organ) << 8 & MASK_ROOT_ID)
        }
        pub fn add_root_coord(organ: Organ, root_coord: Coord) -> Organ {
            organ | (((root_coord as u32) << 8) & MASK_ROOT_ID)
        }
        pub fn get_root_coord(organ: Organ) -> Coord {
            ((organ & MASK_ROOT_ID) >> 8) as Coord
        }
        pub fn get_owner(organ: Organ) -> u8 {
            (organ & MASK_PLAYER) as u8
        }
        pub fn get_direction(organ: Organ) -> OrganDirection {
            OrganDirection::from_index(((organ & MASK_ORGAN_DIRECTION) >> 5) as usize)
        }
        pub fn get_type(organ: Organ) -> OrganType {
            OrganType::from_index(((organ & MASK_ORGAN_TYPE) >> 1) as usize)
        }
        pub fn get_face_coord(organ: Organ, coord: Coord) -> Coord {
            get_direction(organ).move_pos(coord)
        }
        pub fn is_faced_to(organ: Organ, organ_coord: Coord, coord: Coord) -> bool {
            get_face_coord(organ, organ_coord) == coord
        }
        pub fn is_root(organ: Organ) -> bool {
            get_type(organ) == OrganType::Root
        }
        pub fn is_harvester(organ: Organ) -> bool {
            get_type(organ) == OrganType::Harvester
        }
        pub fn is_sporer(organ: Organ) -> bool {
            get_type(organ) == OrganType::Sporer
        }
        pub fn is_tentacle(organ: Organ) -> bool {
            get_type(organ) == OrganType::Tentacle
        }
        pub fn is_basic(organ: Organ) -> bool {
            get_type(organ) == OrganType::Basic
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_entities::coord;
            #[test]
            fn test_new() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                assert_eq!(get_owner(organ), 0);
                assert_eq!(get_type(organ), OrganType::Root);
                assert_eq!(get_direction(organ), OrganDirection::North);
            }
            #[test]
            fn test_add_root_coord() {
                let organ = new(1, OrganType::Tentacle, OrganDirection::East, 0);
                let organ = add_root_coord(organ, 0xFFFF);
                assert_eq!(get_owner(organ), 1);
                assert_eq!(get_type(organ), OrganType::Tentacle);
                assert_eq!(get_direction(organ), OrganDirection::East);
                assert_eq!(0xFFFF, (organ & MASK_ROOT_ID) >> 8);
            }
            #[test]
            fn test_get_owner() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                assert_eq!(get_owner(organ), 0);
            }
            #[test]
            fn test_get_direction() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                assert_eq!(get_direction(organ), OrganDirection::North);
            }
            #[test]
            fn test_get_type() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                assert_eq!(get_type(organ), OrganType::Root);
            }
            #[test]
            fn test_is_faced_to() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                let organ_coord = coord::new(0, 1);
                let coord = coord::new(0, 0);
                assert!(is_faced_to(organ, organ_coord, coord));
            }
            #[test]
            fn test_is_root() {
                let organ = new(0, OrganType::Root, OrganDirection::North, 0);
                assert!(is_root(organ));
            }
            #[test]
            fn test_is_harvester() {
                let organ = new(0, OrganType::Harvester, OrganDirection::North, 0);
                assert!(is_harvester(organ));
            }
        }
    }
    pub mod organ_direction {
        use super::coord::{self, Coord};
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OrganDirection {
            North = 0b000,
            West = 0b001,
            South = 0b010,
            East = 0b011,
            X = 0b100,
        }
        impl OrganDirection {
            pub fn from_str(s: &str) -> OrganDirection {
                match s {
                    "NORTH" | "N" => OrganDirection::North,
                    "WEST" | "W" => OrganDirection::West,
                    "SOUTH" | "S" => OrganDirection::South,
                    "EAST" | "E" => OrganDirection::East,
                    "X" => OrganDirection::X,
                    _ => panic!("Invalid direction {}", s),
                }
            }
            pub fn from_char(c: char) -> OrganDirection {
                match c {
                    'N' => OrganDirection::North,
                    'W' => OrganDirection::West,
                    'S' => OrganDirection::South,
                    'E' => OrganDirection::East,
                    'X' => OrganDirection::X,
                    _ => panic!("Invalid direction {}", c),
                }
            }
            pub fn from_index(i: usize) -> OrganDirection {
                match i {
                    0b000 => OrganDirection::North,
                    0b001 => OrganDirection::West,
                    0b010 => OrganDirection::South,
                    0b011 => OrganDirection::East,
                    0b100 => OrganDirection::X,
                    _ => panic!("Invalid direction index {}", i),
                }
            }
            pub fn to_char(&self) -> char {
                match self {
                    OrganDirection::North => 'N',
                    OrganDirection::West => 'W',
                    OrganDirection::South => 'S',
                    OrganDirection::East => 'E',
                    OrganDirection::X => 'X',
                }
            }
            pub fn opposite(&self) -> OrganDirection {
                match self {
                    OrganDirection::North => OrganDirection::South,
                    OrganDirection::West => OrganDirection::East,
                    OrganDirection::South => OrganDirection::North,
                    OrganDirection::East => OrganDirection::West,
                    OrganDirection::X => OrganDirection::X,
                }
            }
            pub fn turn_left(&self) -> OrganDirection {
                match self {
                    OrganDirection::North => OrganDirection::West,
                    OrganDirection::West => OrganDirection::South,
                    OrganDirection::South => OrganDirection::East,
                    OrganDirection::East => OrganDirection::North,
                    OrganDirection::X => OrganDirection::X,
                }
            }
            pub fn turn_right(&self) -> OrganDirection {
                match self {
                    OrganDirection::North => OrganDirection::East,
                    OrganDirection::West => OrganDirection::North,
                    OrganDirection::South => OrganDirection::West,
                    OrganDirection::East => OrganDirection::South,
                    OrganDirection::X => OrganDirection::X,
                }
            }
            pub fn move_pos(&self, pos: coord::Coord) -> coord::Coord {
                let x = coord::x(pos);
                let y = coord::y(pos);
                match self {
                    OrganDirection::North => coord::new(x, y - 1),
                    OrganDirection::West => coord::new(x - 1, y),
                    OrganDirection::South => coord::new(x, y + 1),
                    OrganDirection::East => coord::new(x + 1, y),
                    OrganDirection::X => coord::new(x, y),
                }
            }
        }
        pub fn found_next_direction(src: Coord, dst: Coord) -> OrganDirection {
            if coord::x(src) < coord::x(dst) {
                return OrganDirection::East;
            }
            if coord::x(src) > coord::x(dst) {
                return OrganDirection::West;
            }
            if coord::y(src) < coord::y(dst) {
                return OrganDirection::South;
            }
            OrganDirection::North
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_organ_direction_from_str() {
                assert_eq!(OrganDirection::from_str("NORTH"), OrganDirection::North);
                assert_eq!(OrganDirection::from_str("WEST"), OrganDirection::West);
                assert_eq!(OrganDirection::from_str("SOUTH"), OrganDirection::South);
                assert_eq!(OrganDirection::from_str("EAST"), OrganDirection::East);
            }
            #[test]
            fn test_organ_direction_from_char() {
                assert_eq!(OrganDirection::from_char('N'), OrganDirection::North);
                assert_eq!(OrganDirection::from_char('W'), OrganDirection::West);
                assert_eq!(OrganDirection::from_char('S'), OrganDirection::South);
                assert_eq!(OrganDirection::from_char('E'), OrganDirection::East);
            }
            #[test]
            fn test_organ_direction_to_char() {
                assert_eq!(OrganDirection::North.to_char(), 'N');
                assert_eq!(OrganDirection::West.to_char(), 'W');
                assert_eq!(OrganDirection::South.to_char(), 'S');
                assert_eq!(OrganDirection::East.to_char(), 'E');
            }
            #[test]
            fn test_organ_direction_opposite() {
                assert_eq!(OrganDirection::North.opposite(), OrganDirection::South);
                assert_eq!(OrganDirection::West.opposite(), OrganDirection::East);
                assert_eq!(OrganDirection::South.opposite(), OrganDirection::North);
                assert_eq!(OrganDirection::East.opposite(), OrganDirection::West);
            }
            #[test]
            fn test_organ_direction_turn_left() {
                assert_eq!(OrganDirection::North.turn_left(), OrganDirection::West);
                assert_eq!(OrganDirection::West.turn_left(), OrganDirection::South);
                assert_eq!(OrganDirection::South.turn_left(), OrganDirection::East);
                assert_eq!(OrganDirection::East.turn_left(), OrganDirection::North);
            }
            #[test]
            fn test_organ_direction_turn_right() {
                assert_eq!(OrganDirection::North.turn_right(), OrganDirection::East);
                assert_eq!(OrganDirection::West.turn_right(), OrganDirection::North);
                assert_eq!(OrganDirection::South.turn_right(), OrganDirection::West);
                assert_eq!(OrganDirection::East.turn_right(), OrganDirection::South);
            }
            #[test]
            fn test_organ_direction_move_pos() {
                let pos = crate::game_entities::coord::new(1, 1);
                assert_eq!(
                    OrganDirection::North.move_pos(pos),
                    crate::game_entities::coord::new(1, 0)
                );
                assert_eq!(
                    OrganDirection::West.move_pos(pos),
                    crate::game_entities::coord::new(0, 1)
                );
                assert_eq!(
                    OrganDirection::South.move_pos(pos),
                    crate::game_entities::coord::new(1, 2)
                );
                assert_eq!(
                    OrganDirection::East.move_pos(pos),
                    crate::game_entities::coord::new(2, 1)
                );
            }
            #[test]
            fn test_found_next_direction() {
                let src = coord::new(1, 1);
                assert_eq!(
                    found_next_direction(src, coord::new(1, 2)),
                    OrganDirection::South
                );
                assert_eq!(
                    found_next_direction(src, coord::new(1, 0)),
                    OrganDirection::North
                );
                assert_eq!(
                    found_next_direction(src, coord::new(0, 1)),
                    OrganDirection::West
                );
                assert_eq!(
                    found_next_direction(src, coord::new(2, 1)),
                    OrganDirection::East
                );
            }
        }
    }
    pub mod organ_owner {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OrganeOwner {
            PlayerOwner = 1,
            EnemyOwner = 0,
            NotOrgan = -1,
        }
        impl OrganeOwner {
            pub fn from_i32(i: i32) -> Option<OrganeOwner> {
                match i {
                    1 => Some(OrganeOwner::PlayerOwner),
                    0 => Some(OrganeOwner::EnemyOwner),
                    -1 => Some(OrganeOwner::NotOrgan),
                    _ => panic!("Invalid owner {} for organ", i),
                }
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_organ_owner_from_i32() {
                assert_eq!(OrganeOwner::from_i32(1), Some(OrganeOwner::PlayerOwner));
                assert_eq!(OrganeOwner::from_i32(0), Some(OrganeOwner::EnemyOwner));
                assert_eq!(OrganeOwner::from_i32(-1), Some(OrganeOwner::NotOrgan));
            }
        }
    }
    pub mod organ_type {
        use super::protein_wallet::ProteinWallet;
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OrganType {
            Basic = 0b0000,
            Harvester = 0b0001,
            Sporer = 0b0010,
            Tentacle = 0b0011,
            Root = 0b0100,
        }
        impl OrganType {
            pub fn from_str(s: &str) -> OrganType {
                match s {
                    "ROOT" => OrganType::Root,
                    "BASIC" => OrganType::Basic,
                    "HARVESTER" => OrganType::Harvester,
                    "SPORER" => OrganType::Sporer,
                    "TENTACLE" => OrganType::Tentacle,
                    _ => panic!("Invalid organ type {}", s),
                }
            }
            pub fn from_index(i: usize) -> OrganType {
                match i {
                    0b0001 => OrganType::Root,
                    0b0010 => OrganType::Basic,
                    0b0011 => OrganType::Harvester,
                    0b0100 => OrganType::Sporer,
                    0b0101 => OrganType::Tentacle,
                    _ => panic!("Invalid organ type index {}", i),
                }
            }
            pub fn to_str(&self) -> &str {
                match self {
                    OrganType::Root => "ROOT",
                    OrganType::Basic => "BASIC",
                    OrganType::Harvester => "HARVESTER",
                    OrganType::Sporer => "SPORER",
                    OrganType::Tentacle => "TENTACLE",
                }
            }
            pub fn get_cost(&self) -> ProteinWallet {
                match self {
                    OrganType::Root => 0x01_01_01_01,
                    OrganType::Basic => 0x01_00_00_00,
                    OrganType::Harvester => 0x00_01_01_00,
                    OrganType::Sporer => 0x00_00_01_01,
                    OrganType::Tentacle => 0x00_01_00_01,
                }
            }
            pub fn get_index(&self) -> u8 {
                match self {
                    OrganType::Root => 0,
                    OrganType::Basic => 1,
                    OrganType::Harvester => 2,
                    OrganType::Sporer => 3,
                    OrganType::Tentacle => 4,
                }
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_organ_type_from_str() {
                assert_eq!(OrganType::from_str("ROOT"), OrganType::Root);
                assert_eq!(OrganType::from_str("BASIC"), OrganType::Basic);
                assert_eq!(OrganType::from_str("HARVESTER"), OrganType::Harvester);
                assert_eq!(OrganType::from_str("SPORER"), OrganType::Sporer);
                assert_eq!(OrganType::from_str("TENTACLE"), OrganType::Tentacle);
            }
            #[test]
            fn test_organ_type_to_str() {
                assert_eq!(OrganType::Root.to_str(), "ROOT");
                assert_eq!(OrganType::Basic.to_str(), "BASIC");
                assert_eq!(OrganType::Harvester.to_str(), "HARVESTER");
                assert_eq!(OrganType::Sporer.to_str(), "SPORER");
                assert_eq!(OrganType::Tentacle.to_str(), "TENTACLE");
            }
        }
    }
    pub mod player {
        use super::{
            coord::Coord,
            organ::{self, Organ},
            protein::Protein,
            protein_wallet::{self, ProteinWallet},
        };
        use std::collections::HashSet;
        pub struct Player {
            id: u8,
            wallet: ProteinWallet,
            organs: HashSet<Coord>,
            roots: HashSet<Coord>,
        }
        impl Player {
            pub fn new(id: u8) -> Self {
                Player {
                    id,
                    wallet: 0,
                    organs: HashSet::new(),
                    roots: HashSet::new(),
                }
            }
            pub fn add_organ(&mut self, coord: Coord, organ: Organ) {
                self.organs.insert(coord);
                if organ::is_root(organ) {
                    self.roots.insert(coord);
                }
            }
            pub fn remove_organ(&mut self, coord: Coord) {
                self.organs.remove(&coord);
                self.roots.remove(&coord);
            }
            pub fn add_root(&mut self, coord: Coord) {
                self.roots.insert(coord);
                self.organs.insert(coord);
            }
            pub fn add_protein(&mut self, protein: Protein, amount: u32) {
                protein_wallet::add(&mut self.wallet, protein, amount);
            }
            pub fn remove_protein(&mut self, protein: Protein, amount: u32) {
                protein_wallet::remove(&mut self.wallet, protein, amount);
            }
            pub fn get_nb_protein(&self, protein: Protein) -> u32 {
                match protein {
                    Protein::A => (self.wallet >> (8 * Protein::A as u32)) & 0xFF,
                    Protein::B => (self.wallet >> (8 * Protein::B as u32)) & 0xFF,
                    Protein::C => (self.wallet >> (8 * Protein::C as u32)) & 0xFF,
                    Protein::D => (self.wallet >> (8 * Protein::D as u32)) & 0xFF,
                }
            }
            pub fn get_id(&self) -> u8 {
                self.id
            }
            pub fn get_roots(&self) -> &HashSet<Coord> {
                &self.roots
            }
            pub fn get_score(&self) -> u32 {
                self.organs.len() as u32
            }
            pub fn get_wallet(&self) -> ProteinWallet {
                self.wallet
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::game_entities::{coord, organ_direction, organ_type};
            #[test]
            fn test_add_organ() {
                let mut player = Player::new(0);
                let coord = coord::new(0, 0);
                let organ = organ::new(
                    0,
                    organ_type::OrganType::Root,
                    organ_direction::OrganDirection::North,
                    0,
                );
                player.add_organ(coord, organ);
                assert!(player.organs.contains(&coord));
                assert!(player.roots.contains(&coord));
            }
            #[test]
            fn test_remove_organ() {
                let mut player = Player::new(0);
                let coord = coord::new(0, 0);
                let organ = organ::new(
                    0,
                    organ_type::OrganType::Root,
                    organ_direction::OrganDirection::North,
                    0,
                );
                player.add_organ(coord, organ);
                player.remove_organ(coord);
                assert!(!player.organs.contains(&coord));
                assert!(!player.roots.contains(&coord));
            }
            #[test]
            fn test_add_root() {
                let mut player = Player::new(0);
                let coord = coord::new(0, 0);
                player.add_root(coord);
                assert!(player.organs.contains(&coord));
                assert!(player.roots.contains(&coord));
            }
            #[test]
            fn test_add_protein() {
                let mut player = Player::new(0);
                player.add_protein(Protein::A, 1);
                assert_eq!(player.get_nb_protein(Protein::A), 1);
            }
            #[test]
            fn test_remove_protein() {
                let mut player = Player::new(0);
                player.add_protein(Protein::A, 1);
                player.remove_protein(Protein::A, 1);
                assert_eq!(player.get_nb_protein(Protein::A), 0);
            }
            #[test]
            fn test_get_nb_protein() {
                let mut player = Player::new(0);
                player.add_protein(Protein::A, 1);
                player.add_protein(Protein::A, 1);
                player.add_protein(Protein::B, 1);
                assert_eq!(player.get_nb_protein(Protein::A), 2);
                assert_eq!(player.get_nb_protein(Protein::B), 1);
                assert_eq!(player.get_nb_protein(Protein::C), 0);
                assert_eq!(player.get_nb_protein(Protein::D), 0);
            }
        }
    }
    pub mod protain_summary {
        use super::protein::Protein;
        use std::collections::HashMap;
        pub struct ProteinSummary {
            from_growth: HashMap<Protein, u32>,
            from_harvest: HashMap<Protein, u32>,
            from_absorb: HashMap<Protein, u32>,
        }
        impl ProteinSummary {
            pub fn new() -> Self {
                ProteinSummary {
                    from_growth: HashMap::new(),
                    from_harvest: HashMap::new(),
                    from_absorb: HashMap::new(),
                }
            }
            pub fn clear(&mut self) {
                self.from_growth.clear();
                self.from_harvest.clear();
                self.from_absorb.clear();
            }
            fn save(report: &mut HashMap<Protein, u32>, protein: Protein, n: u32) {
                report.entry(protein).and_modify(|v| *v += n).or_insert(n);
            }
            pub fn lose_from_growth(&mut self, protein: Protein, n: u32) {
                Self::save(&mut self.from_growth, protein, n);
            }
            pub fn get_from_harvest(&mut self, protein: Protein, n: u32) {
                Self::save(&mut self.from_harvest, protein, n);
            }
            pub fn get_from_absorb(&mut self, protein: Protein, n: u32) {
                Self::save(&mut self.from_absorb, protein, n);
            }
        }
    }
    pub mod protein {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Protein {
            A = 0b00,
            B = 0b01,
            C = 0b10,
            D = 0b11,
        }
        impl Protein {
            pub fn from_str(s: &str) -> Option<Protein> {
                match s {
                    "A" => Some(Protein::A),
                    "B" => Some(Protein::B),
                    "C" => Some(Protein::C),
                    "D" => Some(Protein::D),
                    _ => panic!("Invalid protein type {}", s),
                }
            }
            pub fn to_str(&self) -> &str {
                match self {
                    Protein::A => "A",
                    Protein::B => "B",
                    Protein::C => "C",
                    Protein::D => "D",
                }
            }
            pub fn from_id(id: u8) -> Option<Protein> {
                match id {
                    0 => Some(Protein::A),
                    1 => Some(Protein::B),
                    2 => Some(Protein::C),
                    3 => Some(Protein::D),
                    _ => panic!("Invalid protein id {}", id),
                }
            }
            pub fn to_id(&self) -> u8 {
                match self {
                    Protein::A => 0,
                    Protein::B => 1,
                    Protein::C => 2,
                    Protein::D => 3,
                }
            }
        }
    }
}
macro_rules! parse_input {
    ( $ x : expr , $ t : ident ) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32);
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let _type = inputs[2].trim().to_string();
            let owner = parse_input!(inputs[3], i32);
            let organ_id = parse_input!(inputs[4], i32);
            let organ_dir = inputs[5].trim().to_string();
            let organ_parent_id = parse_input!(inputs[6], i32);
            let organ_root_id = parse_input!(inputs[7], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_a = parse_input!(inputs[0], i32);
        let my_b = parse_input!(inputs[1], i32);
        let my_c = parse_input!(inputs[2], i32);
        let my_d = parse_input!(inputs[3], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opp_a = parse_input!(inputs[0], i32);
        let opp_b = parse_input!(inputs[1], i32);
        let opp_c = parse_input!(inputs[2], i32);
        let opp_d = parse_input!(inputs[3], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let required_actions_count = parse_input!(input_line, i32);
        for i in 0..required_actions_count as usize {
            println!("GROWTH 1 17 8 BASIC N");
        }
    }
}
