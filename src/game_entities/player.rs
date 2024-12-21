use std::collections::HashMap;

use super::{organ::Organ, protain_summary::ProteinSummary, protein::Protein};

pub struct Player {
    storage: HashMap<Protein, u8>,
    organs: Vec<Box<Organ>>,
    roots: Vec<Box<Organ>>,
    protein_summary: ProteinSummary,
}

impl Player {
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
