use std::io;
pub mod game_entities;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
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
use super::{coord::Coord, organ::Organ, player::Player, protein::Protein};

pub struct Cell<'a> {
    pos: Coord,
    obstacle: bool,
    protein: Option<Protein>,
    organ: Option<&'a Organ<'a>>,
}

impl<'a> Cell<'a> {
    pub fn new(
        pos: Coord,
        obstacle: bool,
        protein: Option<Protein>,
        organ: Option<&'a Organ<'a>>,
    ) -> Self {
        Cell {
            pos,
            obstacle,
            protein,
            organ,
        }
    }

    pub fn new_wall(pos: Coord) -> Self {
        Cell {
            pos,
            obstacle: true,
            protein: None,
            organ: None,
        }
    }

    pub fn pos(&self) -> Coord {
        self.pos.clone()
    }

    pub fn get_protein(&self) -> Option<Protein> {
        self.protein
    }

    pub fn get_organ(&self) -> Option<&Organ> {
        self.organ.as_deref()
    }

    pub fn set_protein(&mut self, protein: Protein) {
        self.protein = Some(protein);
    }

    pub fn set_obstacle(&mut self) {
        self.obstacle = true;
        self.protein = None;
    }

    pub fn unset_obstacle(&mut self) {
        self.obstacle = false;
    }

    pub fn place_organ(&mut self, organ: &'a Organ) {
        self.organ = Some(organ);
        self.obstacle = false;
        self.protein = None;
    }

    pub fn obstacle(&self) -> bool {
        self.obstacle
    }

    pub fn is_organ(&self) -> bool {
        self.organ.is_some()
    }

    pub fn is_protein(&self) -> bool {
        self.protein.is_some()
    }

    pub fn is_obstacle(&self) -> bool {
        self.obstacle
    }

    pub fn clear(&mut self) {
        self.obstacle = false;
        self.protein = None;
        self.organ = None;
    }

    pub fn has_protein(&self) -> bool {
        self.protein.is_some()
    }

    pub fn has_organ(&self) -> bool {
        self.organ.is_some()
    }

    pub fn has_tentacle(&self, owner: &Player, facing: Coord) -> bool {
        self.organ.as_ref().map_or(false, |organ| {
            organ.get_owner().eq(&owner) && organ.is_tentacle() && organ.is_faced_to(facing)
        })
    }

    pub fn assign_new_cell(&mut self, cell: &Cell<'a>) {
        self.obstacle = cell.obstacle;
        self.protein = cell.protein;
        self.organ = cell.organ;
    }
}

#[cfg(test)]
mod tests {
    use crate::game_entities::{coord, organ};

    use super::*;

    #[test]
    fn test_cell() {
        let coord = coord::new(0, 0);
        let cell = Cell::new(coord, false, Some(Protein::A), None);
        assert_eq!(cell.pos(), coord);
        assert_eq!(cell.obstacle(), false);
        assert_eq!(cell.get_protein(), Some(Protein::A));
        assert_eq!(cell.is_organ(), false);
        assert_eq!(cell.is_protein(), true);
    }

    #[test]
    fn test_cell_wall() {
        let coord = coord::new(0, 0);
        let cell = Cell::new(coord, true, None, None);
        assert_eq!(cell.pos(), coord);
        assert_eq!(cell.obstacle(), true);
        assert_eq!(cell.get_protein(), None);
        assert_eq!(cell.is_organ(), false);
        assert_eq!(cell.is_protein(), false);
    }
}
use std::collections::HashMap;

use super::{organ::Organ, protain_summary::ProteinSummary, protein::Protein};

pub struct Player<'a> {
    storage: HashMap<Protein, u8>,
    organs: Vec<Box<Organ<'a>>>,
    roots: Vec<Box<Organ<'a>>>,
    protein_summary: ProteinSummary,
}

impl Player<'_> {
    pub fn new() -> Self {
        Player {
            storage: HashMap::new(),
            organs: Vec::new(),
            roots: Vec::new(),
            protein_summary: ProteinSummary::new(),
        }
    }

    pub fn eq(&self, other: &Player) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }
        false
    }
}
use std::collections::HashMap;

use super::protein::Protein;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganType {
    Root = 0b1111,
    Basic = 0b1000,
    Harvester = 0b0110,
    Sporer = 0b0011,
    Tentacle = 0b0101,
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

    pub fn to_str(&self) -> &str {
        match self {
            OrganType::Root => "ROOT",
            OrganType::Basic => "BASIC",
            OrganType::Harvester => "HARVESTER",
            OrganType::Sporer => "SPORER",
            OrganType::Tentacle => "TENTACLE",
        }
    }

    pub fn get_cost(&self) -> u32 {
        match self {
            OrganType::Root => 0,
            OrganType::Basic => 10,
            OrganType::Harvester => 20,
            OrganType::Sporer => 30,
            OrganType::Tentacle => 40,
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
pub mod cell;
pub mod coord;
pub mod grid;
pub mod organ;
pub mod organ_direction;
pub mod organ_owner;
pub mod organ_type;
pub mod player;
pub mod protain_summary;
pub mod protein;
pub type Coord = u16;

pub fn new(x: u8, y: u8) -> Coord {
    (x as u16) << 8 | y as u16
}

pub fn x(coord: Coord) -> u8 {
    (coord >> 8) as u8
}

pub fn y(coord: Coord) -> u8 {
    (coord & 0xff) as u8
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
    fn test_pos_new() {
        let pos = new(1, 2);
        assert_eq!(x(pos), 1);
        assert_eq!(y(pos), 2);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protein {
    A,
    B,
    C,
    D,
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
use super::{cell::Cell, coord};

pub struct Grid<'a> {
    cells: Vec<Cell<'a>>,
    width: u8,
    height: u8,
}

impl<'a> Grid<'a> {
    pub fn new(width: u8, height: u8) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new_wall(coord::new(x, y)));
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> &Cell {
        if x < self.width && y < self.height {
            &self.cells[x as usize + self.width as usize * y as usize]
        } else {
            panic!("Cell out of bounds");
        }
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: &Cell<'a>) {
        self.cells[x as usize + self.width as usize * y as usize].assign_new_cell(cell);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.get_cell(0, 0).is_obstacle(), true);
        assert_eq!(grid.get_cell(1, 1).is_obstacle(), true);
        assert_eq!(grid.get_cell(2, 2).is_obstacle(), true);
    }
}
use super::coord::{self, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganDirection {
    North,
    West,
    South,
    East,
    X,
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
        match self {
            OrganDirection::North => coord::new(coord::x(pos), coord::y(pos) - 1),
            OrganDirection::West => coord::new(coord::x(pos) - 1, coord::y(pos)),
            OrganDirection::South => coord::new(coord::x(pos), coord::y(pos) + 1),
            OrganDirection::East => coord::new(coord::x(pos) + 1, coord::y(pos)),
            OrganDirection::X => coord::new(coord::x(pos), coord::y(pos)),
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
pub mod game_entities;
