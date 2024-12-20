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
    fn test_cell_organ() {
        let binding = Player::new();
        let organ = organ::Organ::new_from_input(1, 1, "A", &binding, 1, "S", 1, 1);
        let coord = coord::new(0, 0);
        let cell = Cell::new(coord, false, None, Some(&organ));
        assert_eq!(cell.pos(), coord);
        assert_eq!(cell.obstacle(), false);
        assert_eq!(cell.get_protein(), None);
        assert_eq!(cell.get_organ().unwrap().get_id(), 1);
        assert_eq!(cell.is_organ(), true);
        assert_eq!(cell.is_protein(), false);
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
