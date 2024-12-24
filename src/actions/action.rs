use crate::game_entities::{
    coord::Coord, grid::Grid, organ, organ_direction::OrganDirection, organ_type::OrganType,
    player::Player, protein_wallet,
};

use super::action_type::{self, ActionType};

const MASK_TYPE: Action = 0b0000_0011;
const MASK_ORGAN: Action = 0b0001_1100;
const MASK_DIRECTION: Action = 0b0110_0000;
const MASK_COORD_TARGET: Action = 0b0000_0000_1111_1111_1000_0000;
const MASK_COORD_ROOT: Action = 0b1_1111_1111_0000_0000_0000_0000;

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
        && grid.can_add_organ(
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
