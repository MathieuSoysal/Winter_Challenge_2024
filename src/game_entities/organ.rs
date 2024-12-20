use super::{
    coord::{self, Coord},
    organ_direction::OrganDirection,
    organ_type::OrganType,
    player::Player,
};

pub struct Organ<'a> {
    id: u32,
    organ_type: OrganType,
    owner: &'a Player<'a>,
    parent: u32,
    children: Vec<Box<Organ<'a>>>,
    direction: OrganDirection,
    pos: Coord,
    root_id: u32,
}

impl<'a> Organ<'a> {
    pub fn new(owner: &'a Player<'a>, organ_type: OrganType, direction: OrganDirection) -> Self {
        Organ {
            id: 1,
            organ_type,
            owner: owner,
            parent: 0,
            children: Vec::new(),
            direction: direction,
            pos: coord::new(0, 0),
            root_id: 1,
        }
    }

    pub fn new_from_input(
        x: i32,
        y: i32,
        type_organ: &'a str,
        owner: &'a Player<'a>,
        organ_id: i32,
        direction: &'a str,
        organ_parent_id: i32,
        organ_root_id: i32,
    ) -> Self {
        let organ_type = OrganType::from_str(type_organ);
        let organ_direction = OrganDirection::from_str(direction);
        let mut result = Organ::new(owner, organ_type, organ_direction);
        result.id = organ_id as u32;
        result.pos = coord::new(x as u8, y as u8);
        result.root_id = organ_root_id as u32;
        if organ_parent_id != 0 {
            result.parent = organ_parent_id as u32;
        }
        result
    }

    pub fn hash(&self) -> u16 {
        self.pos
    }

    pub fn get_owner(&self) -> &Player {
        self.owner
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_parent_id(&self) -> u32 {
        self.parent
    }

    pub fn get_root_id(&self) -> u32 {
        self.root_id
    }

    pub fn get_pos(&self) -> Coord {
        self.pos
    }

    pub fn get_direction(&self) -> OrganDirection {
        self.direction
    }

    pub fn get_type(&self) -> OrganType {
        self.organ_type
    }

    pub fn get_face_coord(&self) -> Coord {
        self.direction.move_pos(self.pos)
    }

    pub fn is_faced_to(&self, coord: Coord) -> bool {
        self.get_face_coord() == coord
    }

    pub fn is_nucleus(&self) -> bool {
        self.organ_type == OrganType::Root
    }

    pub fn is_harvester(&self) -> bool {
        self.organ_type == OrganType::Harvester
    }

    pub fn is_sporer(&self) -> bool {
        self.organ_type == OrganType::Sporer
    }

    pub fn is_tentacle(&self) -> bool {
        self.organ_type == OrganType::Tentacle
    }
}
