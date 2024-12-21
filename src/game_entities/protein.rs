#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protein {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
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
