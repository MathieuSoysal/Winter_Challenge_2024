use crate::game_entities::{
    coord, grid::Grid, organ, organ_direction::OrganDirection, organ_type::OrganType,
};

use super::{action, action_type::ActionType};

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

pub fn make_sporer_valid(
    action_last_sporer_creation: action::Action,
    action: action::Action,
    grid: &Grid,
) -> action::Action {
    let x_coord = coord::x(action::get_coord_target(action_last_sporer_creation));
    let y_coord = coord::y(action::get_coord_target(action_last_sporer_creation));
    let sporer_direction = action::get_direction(action_last_sporer_creation);
    let mut result = action;
    result = action::set_coord_target(result, coord::new(x_coord, y_coord));
    result = action::set_coord_source(
        result,
        action::get_coord_source(action_last_sporer_creation),
    );

    match sporer_direction {
        OrganDirection::East => {
            if x_coord >= grid.width - 1 {
                return action::wait();
            }
            let adition_on_x = (rand::random::<u8>() % (grid.width - 1 - x_coord)) + 1;
            action::set_coord_target(result, coord::new(x_coord + adition_on_x, y_coord))
        }
        OrganDirection::West => {
            if x_coord == 0 {
                return action::wait();
            }
            let substraction_on_x = (rand::random::<u8>() % x_coord) + 1;
            action::set_coord_target(result, coord::new(x_coord - substraction_on_x, y_coord))
        }
        OrganDirection::North => {
            if y_coord == 0 {
                return action::wait();
            }
            let substraction_on_y = (rand::random::<u8>() % y_coord) + 1;
            action::set_coord_target(result, coord::new(x_coord, y_coord - substraction_on_y))
        }
        OrganDirection::South => {
            if y_coord >= grid.height - 1 {
                return action::wait();
            }
            let adition_on_y = rand::random::<u8>() % (grid.height - y_coord);
            action::set_coord_target(result, coord::new(x_coord, y_coord + adition_on_y))
        }
        _ => action::wait(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_entities::{cell, organ_direction::OrganDirection, organ_type::OrganType};

    #[test]
    fn test_make_it_valid() {
        let grid = Grid::new(5, 5);
        let action = action::growth(
            OrganType::Basic,
            action::get_direction(action::wait()),
            coord::new(0, 0),
            coord::new(0, 0),
        );
        let valid_action = make_growth_valid(action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Wait);
    }

    #[test]
    fn test_make_it_valid_with_a_valid_move() {
        let mut grid = Grid::new(5, 5);
        let root_coord = coord::new(0, 0);
        let cell = cell::new(
            false,
            None,
            Some(organ::new(
                0,
                OrganType::Root,
                OrganDirection::X,
                root_coord,
            )),
        );
        grid.set_cell(0, 4, cell);
        let action = action::growth(
            OrganType::Basic,
            OrganDirection::South,
            coord::new(0, 0),
            coord::new(0, 0),
        );
        let valid_action = make_growth_valid(action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Growth);
        assert_eq!(action::get_direction(valid_action), OrganDirection::South);
        assert_eq!(action::get_coord_target(valid_action), coord::new(0, 3));
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
    fn test_make_growth_valid_with_a_valid_move() {
        let mut grid = Grid::new(5, 5);
        let root_coord = coord::new(0, 0);
        let cell = cell::new(
            false,
            None,
            Some(organ::new(
                0,
                OrganType::Root,
                OrganDirection::X,
                root_coord,
            )),
        );
        grid.set_cell(4, 0, cell);
        let action = action::growth(
            OrganType::Root,
            OrganDirection::East,
            coord::new(0, 0),
            coord::new(0, 0),
        );
        let valid_action = make_growth_valid(action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Growth);
        assert_eq!(action::get_direction(valid_action), OrganDirection::East);
        assert_eq!(action::get_coord_target(valid_action), coord::new(3, 0));
    }

    #[test]
    fn test_make_sporer_valid_should_false_beceause_on_edge() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::East, coord::new(4, 4), coord::new(0, 0));
        let action = action::sporer(OrganDirection::East, coord::new(4, 4), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Wait);
    }

    #[test]
    fn test_make_sporer_valid_should_false_beceause_on_edge_2() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::West, coord::new(0, 0), coord::new(0, 0));
        let action = action::sporer(OrganDirection::West, coord::new(0, 0), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Wait);
    }

    #[test]
    fn test_make_sporer_valid_should_false_beceause_on_edge_3() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::North, coord::new(0, 0), coord::new(0, 0));
        let action = action::sporer(OrganDirection::North, coord::new(0, 0), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Wait);
    }

    #[test]
    fn test_make_sporer_valid_should_false_beceause_on_edge_4() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::South, coord::new(4, 4), coord::new(0, 0));
        let action = action::sporer(OrganDirection::South, coord::new(4, 4), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Wait);
    }

    #[test]
    fn test_make_sporer_valid_should_move_to_the_right() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::East, coord::new(3, 3), coord::new(0, 0));
        let action = action::sporer(OrganDirection::East, coord::new(0, 0), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Sporer);
        assert_eq!(action::get_direction(valid_action), OrganDirection::East);
        assert_eq!(action::get_coord_target(valid_action), coord::new(4, 3));
    }

    #[test]
    fn test_make_sporer_valid_should_move_to_the_left() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::West, coord::new(1, 1), coord::new(0, 0));
        let action = action::sporer(OrganDirection::West, coord::new(0, 0), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Sporer);
        assert_eq!(action::get_direction(valid_action), OrganDirection::West);
        assert_eq!(action::get_coord_target(valid_action), coord::new(0, 1));
    }

    #[test]
    fn test_make_sporer_valid_should_move_to_the_top() {
        let grid = Grid::new(5, 5);
        let last_action = action::sporer(OrganDirection::North, coord::new(1, 1), coord::new(0, 0));
        let action = action::sporer(OrganDirection::North, coord::new(0, 0), coord::new(0, 0));
        let valid_action = make_sporer_valid(last_action, action, &grid);
        assert_eq!(action::get_type(valid_action), ActionType::Sporer);
        assert_eq!(action::get_direction(valid_action), OrganDirection::North);
        assert_eq!(action::get_coord_target(valid_action), coord::new(1, 0));
    }
}
