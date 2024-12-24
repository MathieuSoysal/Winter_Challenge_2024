use super::{organ_type::OrganType, protein::Protein};

pub type ProteinWallet = u32;

const MASK_PROTEIN: ProteinWallet = 0xff;
const BITS_PROTEIN: u32 = 8;

pub fn new() -> ProteinWallet {
    0
}

pub fn add(wallet: &mut ProteinWallet, protein_type: Protein, amount: u32) {
    *wallet += amount << (protein_type as ProteinWallet * BITS_PROTEIN);
}

pub fn remove(wallet: &mut ProteinWallet, protein_type: Protein, amount: u32) {
    *wallet -= amount << (protein_type as ProteinWallet * BITS_PROTEIN);
}

pub fn get(wallet: ProteinWallet, protein_type: Protein) -> u8 {
    ((wallet >> (protein_type as ProteinWallet * BITS_PROTEIN)) & MASK_PROTEIN) as u8
}

pub fn can_buy_organ(wallet: ProteinWallet, organ_type: OrganType) -> bool {
    let cost = organ_type.get_cost();

    get(wallet, Protein::A) >= get(cost, Protein::A)
        && get(wallet, Protein::B) >= get(cost, Protein::B)
        && get(wallet, Protein::C) >= get(cost, Protein::C)
        && get(wallet, Protein::D) >= get(cost, Protein::D)
}

pub fn buy_organ(wallet: &mut ProteinWallet, organ_type: OrganType) {
    let cost = organ_type.get_cost();

    remove(wallet, Protein::A, get(cost, Protein::A) as ProteinWallet);
    remove(wallet, Protein::B, get(cost, Protein::B) as ProteinWallet);
    remove(wallet, Protein::C, get(cost, Protein::C) as ProteinWallet);
    remove(wallet, Protein::D, get(cost, Protein::D) as ProteinWallet);
}
