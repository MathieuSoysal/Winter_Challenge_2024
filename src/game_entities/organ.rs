use super::{coord::Coord, organ_direction::OrganDirection, organ_type::OrganType};

const MASK_PLAYER: u16 = 0b0000_0001;
const MASK_ORGAN_TYPE: u16 = 0b0001_1110;
const MASK_ORGAN_DIRECTION: u16 = 0b1110_0000;

pub type Organ = u16;

pub fn new(owner: u8, organ_type: OrganType, organ_direction: OrganDirection) -> Organ {
    (owner as u16 & MASK_PLAYER)
        | ((organ_type as u16) << 1 & MASK_ORGAN_TYPE)
        | ((organ_direction as u16) << 5 & MASK_ORGAN_DIRECTION)
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
