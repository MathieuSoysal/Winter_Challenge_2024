use super::{coord::Coord, organ_direction::OrganDirection, organ_type::OrganType};

const MASK_PLAYER: Organ = 0b0000_0001;
const MASK_ORGAN_TYPE: Organ = 0b0001_1110;
const MASK_ORGAN_DIRECTION: Organ = 0b1110_0000;
const MASK_ROOT_ID: Organ = 0xFF00;

pub type Organ = u32;

pub fn new(owner: u8, organ_type: OrganType, organ_direction: OrganDirection) -> Organ {
    (owner as Organ & MASK_PLAYER)
        | ((organ_type as Organ) << 1 & MASK_ORGAN_TYPE)
        | ((organ_direction as Organ) << 5 & MASK_ORGAN_DIRECTION)
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
    use crate::game_entities::coord;

    use super::*;

    #[test]
    fn test_new() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        assert_eq!(get_owner(organ), 0);
        assert_eq!(get_type(organ), OrganType::Root);
        assert_eq!(get_direction(organ), OrganDirection::North);
    }

    #[test]
    fn test_add_root_coord() {
        let organ = new(1, OrganType::Tentacle, OrganDirection::East);
        let organ = add_root_coord(organ, 0xFFFF);
        assert_eq!(get_owner(organ), 1);
        assert_eq!(get_type(organ), OrganType::Tentacle);
        assert_eq!(get_direction(organ), OrganDirection::East);
        assert_eq!(0xFFFF, (organ & MASK_ROOT_ID) >> 8);
    }

    #[test]
    fn test_get_owner() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        assert_eq!(get_owner(organ), 0);
    }

    #[test]
    fn test_get_direction() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        assert_eq!(get_direction(organ), OrganDirection::North);
    }

    #[test]
    fn test_get_type() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        assert_eq!(get_type(organ), OrganType::Root);
    }

    #[test]
    fn test_is_faced_to() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        let organ_coord = coord::new(0, 1);
        let coord = coord::new(0, 0);
        assert!(is_faced_to(organ, organ_coord, coord));
    }

    #[test]
    fn test_is_root() {
        let organ = new(0, OrganType::Root, OrganDirection::North);
        assert!(is_root(organ));
    }

    #[test]
    fn test_is_harvester() {
        let organ = new(0, OrganType::Harvester, OrganDirection::North);
        assert!(is_harvester(organ));
    }
}
