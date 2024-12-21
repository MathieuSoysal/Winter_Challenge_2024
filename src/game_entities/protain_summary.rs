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
