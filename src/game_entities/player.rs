use std::collections::HashSet;

use super::{
    coord::Coord,
    organ::{self, Organ},
    protain_summary::ProteinSummary,
    protein::Protein,
};

pub struct Player {
    storage: u32,
    organs: HashSet<Coord>, //TODO : is it opti to do that ?
    roots: HashSet<Coord>,
    protein_summary: ProteinSummary,
}

impl Player {
    pub fn new() -> Self {
        Player {
            storage: 0,
            organs: HashSet::new(),
            roots: HashSet::new(),
            protein_summary: ProteinSummary::new(),
        }
    }

    pub fn add_organ(&mut self, coord: Coord, organ: Organ) {
        self.organs.insert(coord);
        if organ::is_root(organ) {
            self.roots.insert(coord);
        }
    }

    pub fn remove_organ(&mut self, coord: Coord) {
        self.organs.remove(&coord);
        self.roots.remove(&coord);
    }

    pub fn add_root(&mut self, coord: Coord) {
        self.roots.insert(coord);
        self.organs.insert(coord);
    }

    pub fn add_protein(&mut self, protein: Protein) {
        match protein {
            Protein::A => self.storage += 1 << (8 * Protein::A as u32),
            Protein::B => self.storage += 1 << (8 * Protein::B as u32),
            Protein::C => self.storage += 1 << (8 * Protein::C as u32),
            Protein::D => self.storage += 1 << (8 * Protein::D as u32),
        }
    }

    pub fn remove_protein(&mut self, protein: Protein) {
        match protein {
            Protein::A => self.storage -= 1 << (8 * Protein::A as u32),
            Protein::B => self.storage -= 1 << (8 * Protein::B as u32),
            Protein::C => self.storage -= 1 << (8 * Protein::C as u32),
            Protein::D => self.storage -= 1 << (8 * Protein::D as u32),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_entities::{coord, organ_direction, organ_type};

    use super::*;

    #[test]
    fn test_add_organ() {
        let mut player = Player::new();
        let coord = coord::new(0, 0);
        let organ = organ::new(
            0,
            organ_type::OrganType::Root,
            organ_direction::OrganDirection::North,
        );
        player.add_organ(coord, organ);
        assert!(player.organs.contains(&coord));
        assert!(player.roots.contains(&coord));
    }

    #[test]
    fn test_remove_organ() {
        let mut player = Player::new();
        let coord = coord::new(0, 0);
        let organ = organ::new(
            0,
            organ_type::OrganType::Root,
            organ_direction::OrganDirection::North,
        );
        player.add_organ(coord, organ);
        player.remove_organ(coord);
        assert!(!player.organs.contains(&coord));
        assert!(!player.roots.contains(&coord));
    }
}
