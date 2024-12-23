use crate::game_entities::{coord::Coord, organ_direction::OrganDirection, organ_type::OrganType};

use super::action_type::{self, ActionType};

const MASK_TYPE: Action = 0b0000_0011;
const MASK_ORGAN: Action = 0b0001_1100;
const MASK_DIRECTION: Action = 0b0110_0000;
const MASK_COORD_TARGET: Action = 0b0000_0000_1111_1111_1000_0000;
const MASK_COORD_SOURCE: Action = 0b1_1111_1111_0000_0000_0000_0000;

pub type Action = u32;

pub fn new(
    action_type: ActionType,
    organ_type: OrganType,
    direction: OrganDirection,
    coord_target: Coord,
    coord_source: Coord,
) -> Action {
    let action_type = action_type as Action;
    let organ_type = (organ_type as Action) << 2;
    let direction = (direction as Action) << 5;
    let coord_target = (coord_target as Action) << 7;
    let coord_source = (coord_source as Action) << 16;
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
    ((action & MASK_COORD_SOURCE) >> 16) as Coord
}

#[cfg(test)]
mod tests {
    use super::action_type::ActionType;
    use super::*;
    use crate::game_entities::coord;
    use crate::game_entities::organ_direction::OrganDirection;
    use crate::game_entities::organ_type::OrganType;

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
}
