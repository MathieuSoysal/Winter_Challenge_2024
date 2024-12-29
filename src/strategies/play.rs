use std::collections::HashSet;

use crate::{
    actions::action::{self, Action},
    game_entities::{
        cell,
        coord::{self, Coord},
        grid::Grid,
        organ,
        organ_direction::{self, OrganDirection},
        organ_type::OrganType,
        player::Player,
    },
};

// pub fn get_best_action(grid: &Grid, player: &Player, root_coord: Coord) -> Action {}

pub fn found_an_action(grid: &Grid, player: &Player, root_coord: Coord) -> Action {
    let mut possible_coords = HashSet::new();
    fill_all_possible_coord(&mut possible_coords, root_coord, root_coord, grid);

    let mut action: Action = action::wait();
    for coord in possible_coords {
        if player.can_buy(OrganType::Tentacle) {
            if let Some(direction) = can_attack_enemy_organ_in_one_cell(grid, player, coord) {
                return action::growth(OrganType::Tentacle, direction, coord, root_coord);
            }
            if let Some(direction) = can_attack_enemy_organ_in_two_cell(grid, player, coord) {
                action = action::growth(OrganType::Tentacle, direction, coord, root_coord);
            }
        } else if player.can_buy(OrganType::Harvester) {
            if let Some(direction) = grid.get_direction_to_an_adjacent_protein(coord) {
                return action::growth(OrganType::Harvester, direction, coord, root_coord);
            }
        } else if player.can_buy(OrganType::Basic) {
            return action::growth(OrganType::Root, OrganDirection::North, coord, root_coord);
        }
    }
    action
}

pub fn fill_all_possible_coord(
    possible_coord: &mut HashSet<Coord>,
    current_coord: Coord,
    root_coord: Coord,
    grid: &Grid,
) {
    let organ = organ::new(0, OrganType::Basic, OrganDirection::North, root_coord);
    if let Some(children) = grid.get_children(current_coord) {
        for child in children {
            fill_all_possible_coord(possible_coord, *child, root_coord, grid);
            let adjacent_coords = grid.get_adjacent_coords(*child);
            for adjacent_coord in adjacent_coords {
                if grid.can_add_organ_with_root_coord(adjacent_coord, organ) {
                    possible_coord.insert(adjacent_coord);
                }
            }
        }
    }
}

fn can_attack_enemy_organ_in_one_cell(
    grid: &Grid,
    player: &Player,
    coord: Coord,
) -> Option<OrganDirection> {
    let organ = organ::new(
        player.get_id(),
        OrganType::Basic,
        OrganDirection::North,
        coord,
    );
    if grid.can_add_organ_without_root_coord(coord, organ)
        && grid.contains_an_adjacent_organ(
            coord::x(coord),
            coord::y(coord),
            player.get_opponent_id(),
        )
    {
        return grid.get_direction_to_an_adjacent_organ(coord, player.get_opponent_id());
    }
    None
}

fn can_attack_enemy_organ_in_two_cell(
    grid: &Grid,
    player: &Player,
    coord: Coord,
) -> Option<OrganDirection> {
    let organ = organ::new(
        player.get_id(),
        OrganType::Basic,
        OrganDirection::North,
        coord,
    );
    let mut result = None;
    for adj_coord in grid.get_adjacent_coords(coord) {
        if grid.can_add_organ_with_root_coord(adj_coord, organ) {
            for opp_coord in grid.get_adjacent_coords(adj_coord) {
                if cell::is_owned_by(
                    grid.get_cell(coord::x(opp_coord), coord::y(opp_coord)),
                    player.get_opponent_id(),
                ) {
                    result = Some(organ_direction::get_direction_from_coord_to_coord(
                        coord, opp_coord, grid,
                    ));
                    if coord::x(opp_coord) == coord::x(coord)
                        || coord::y(opp_coord) == coord::y(coord)
                    {
                        return result;
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_entities::organ_type::OrganType;
    use crate::game_entities::{cell, coord, organ_direction};

    #[test]
    fn test_can_attack_enemy_organ_in_one_cell() {
        let mut grid = Grid::new(3, 3);
        let player = Player::new(1);
        let coord = coord::new(1, 1);
        let organ_own = organ::new(1, OrganType::Basic, OrganDirection::North, coord);
        let organ_opp = organ::new(0, OrganType::Basic, OrganDirection::North, coord);

        grid.set_cell(0, 1, cell::new(false, None, Some(organ_own)));

        grid.set_cell(1, 2, cell::new(false, None, Some(organ_opp)));
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord),
            Some(organ_direction::OrganDirection::South)
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(1, 2)),
            None
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(1, 0)),
            None
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(0, 1)),
            None
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(2, 1)),
            None
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(0, 0)),
            None
        );
        assert_eq!(
            can_attack_enemy_organ_in_one_cell(&grid, &player, coord::new(0, 2)),
            Some(organ_direction::OrganDirection::East)
        );
    }

    #[test]
    fn test_can_attack_enemy_organ_in_two_cell() {
        let mut grid = Grid::new(3, 3);
        let player = Player::new(1);
        let coord = coord::new(1, 0);
        let organ_own = organ::new(1, OrganType::Basic, OrganDirection::North, coord);
        let organ_opp = organ::new(0, OrganType::Basic, OrganDirection::North, coord);

        grid.set_cell(0, 0, cell::new(false, None, Some(organ_own)));

        grid.set_cell(1, 2, cell::new(false, None, Some(organ_opp)));
        assert_eq!(
            can_attack_enemy_organ_in_two_cell(&grid, &player, coord),
            Some(organ_direction::OrganDirection::South)
        );
    }
}
