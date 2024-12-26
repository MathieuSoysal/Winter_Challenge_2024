use crate::game_entities::{coord, grid::Grid, organ, organ_type::OrganType};

use super::{action, action_type::ActionType};

pub fn make_it_valid(action: action::Action, grid: &Grid) -> action::Action {
    match action::get_type(action) {
        ActionType::Growth => make_growth_valid(action, grid),
        ActionType::Sporer => action::wait(), //TODO make_sporer_valid(action, grid),
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
        let valid_action = make_it_valid(action, &grid);
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
        let valid_action = make_it_valid(action, &grid);
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
}
