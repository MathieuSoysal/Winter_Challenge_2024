use super::protein::Protein;

pub type Cell = u16;

const EMPTY: Cell = 0b00;
const OBSTACLE: Cell = 0b01;
const PROTEIN: Cell = 0b10;
const ORGAN: Cell = 0b11;

const MASK_TYPE: Cell = 0b11;
const MASK_PROTEIN: Cell = 0b11111100;
const MASK_ORGAN: Cell = 0b11111100;

pub fn new(obstacle: bool, protein: Option<Protein>, organ_id: Option<u16>) -> Cell {
    match (obstacle, protein, organ_id) {
        (true, _, _) => OBSTACLE,
        (false, None, None) => EMPTY,
        (false, Some(protein), None) => (protein as u16) << 2 | PROTEIN,
        (false, None, Some(organ_id)) => organ_id << 2 | ORGAN,
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

pub fn get_organ_id(cell: Cell) -> Option<u16> {
    if contains_organ(cell) {
        Some(cell >> 2)
    } else {
        None
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
}
