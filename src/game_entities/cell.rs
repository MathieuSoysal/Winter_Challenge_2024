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
    is_owned_by(cell, owner) && organ::get_root_coord(get_organ(cell).unwrap()) == root_coord
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

// pub fn has_tentacle(&self, owner: &Player, facing: Coord) -> bool {
//     self.organ.as_ref().map_or(false, |organ| {
//         organ.get_owner().eq(&owner) && organ.is_tentacle() && organ.is_faced_to(facing)
//     })
// }

#[cfg(test)]
mod tests {
    use crate::game_entities::cell;

    use super::*;

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
