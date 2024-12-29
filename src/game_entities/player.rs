use std::collections::HashSet;

use super::{
    coord::Coord,
    organ::{self, Organ},
    organ_type::OrganType,
    protein::Protein,
    protein_wallet::{self, ProteinWallet},
};

pub struct Player {
    id: u8,
    wallet: ProteinWallet,
    organs: HashSet<Coord>, //TODO : is it opti to do that ?
    roots: HashSet<Coord>,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            wallet: 0,
            organs: HashSet::new(),
            roots: HashSet::new(),
        }
    }

    pub fn add_organ(&mut self, coord: Coord, organ: Organ) {
        self.organs.insert(coord);
        if organ::is_root(organ) {
            self.roots.insert(coord);
        }
    }

    pub fn can_buy(&self, organ_type: OrganType) -> bool {
        protein_wallet::can_buy_organ(self.wallet, organ_type)
    }

    pub fn get_opponent_id(&self) -> u8 {
        if self.id == 0 {
            1
        } else {
            0
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

    pub fn add_protein(&mut self, protein: Protein, amount: u32) {
        protein_wallet::add(&mut self.wallet, protein, amount);
    }

    pub fn remove_protein(&mut self, protein: Protein, amount: u32) {
        protein_wallet::remove(&mut self.wallet, protein, amount);
    }

    pub fn get_nb_protein(&self, protein: Protein) -> u32 {
        match protein {
            Protein::A => (self.wallet >> (8 * Protein::A as u32)) & 0xFF,
            Protein::B => (self.wallet >> (8 * Protein::B as u32)) & 0xFF,
            Protein::C => (self.wallet >> (8 * Protein::C as u32)) & 0xFF,
            Protein::D => (self.wallet >> (8 * Protein::D as u32)) & 0xFF,
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_roots(&self) -> &HashSet<Coord> {
        &self.roots
    }

    pub fn get_score(&self) -> u32 {
        self.organs.len() as u32
    }

    pub fn get_wallet(&self) -> ProteinWallet {
        self.wallet
    }
}

#[cfg(test)]
mod tests {
    use crate::game_entities::{coord, organ_direction, organ_type};

    use super::*;

    #[test]
    fn test_add_organ() {
        let mut player = Player::new(0);
        let coord = coord::new(0, 0);
        let organ = organ::new(
            0,
            organ_type::OrganType::Root,
            organ_direction::OrganDirection::North,
            0,
        );
        player.add_organ(coord, organ);
        assert!(player.organs.contains(&coord));
        assert!(player.roots.contains(&coord));
    }

    #[test]
    fn test_remove_organ() {
        let mut player = Player::new(0);
        let coord = coord::new(0, 0);
        let organ = organ::new(
            0,
            organ_type::OrganType::Root,
            organ_direction::OrganDirection::North,
            0,
        );
        player.add_organ(coord, organ);
        player.remove_organ(coord);
        assert!(!player.organs.contains(&coord));
        assert!(!player.roots.contains(&coord));
    }

    #[test]
    fn test_add_root() {
        let mut player = Player::new(0);
        let coord = coord::new(0, 0);
        player.add_root(coord);
        assert!(player.organs.contains(&coord));
        assert!(player.roots.contains(&coord));
    }

    #[test]
    fn test_add_protein() {
        let mut player = Player::new(0);
        player.add_protein(Protein::A, 1);
        assert_eq!(player.get_nb_protein(Protein::A), 1);
    }

    #[test]
    fn test_remove_protein() {
        let mut player = Player::new(0);
        player.add_protein(Protein::A, 1);
        player.remove_protein(Protein::A, 1);
        assert_eq!(player.get_nb_protein(Protein::A), 0);
    }

    #[test]
    fn test_get_nb_protein() {
        let mut player = Player::new(0);
        player.add_protein(Protein::A, 1);
        player.add_protein(Protein::A, 1);
        player.add_protein(Protein::B, 1);
        assert_eq!(player.get_nb_protein(Protein::A), 2);
        assert_eq!(player.get_nb_protein(Protein::B), 1);
        assert_eq!(player.get_nb_protein(Protein::C), 0);
        assert_eq!(player.get_nb_protein(Protein::D), 0);
    }
}
