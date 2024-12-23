#[derive(Debug, PartialEq)]
pub enum ActionType {
    Wait = 0b00,
    Growth = 0b01,
    Sporer = 0b10,
}

// trait Action {
// fn get_type(&self) -> ActionType;
// fn get_cost(&self) -> u32;
// fn get_index(&self) -> u8;
// }

impl ActionType {
    pub fn from_str(s: &str) -> ActionType {
        match s {
            "WAIT" => ActionType::Wait,
            "GROWTH" => ActionType::Growth,
            "SPORER" => ActionType::Sporer,
            _ => panic!("Invalid action type {}", s),
        }
    }

    pub fn from_index(i: usize) -> ActionType {
        match i {
            0b00 => ActionType::Wait,
            0b01 => ActionType::Growth,
            0b10 => ActionType::Sporer,
            _ => panic!("Invalid action type index {}", i),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ActionType::Wait => "WAIT",
            ActionType::Growth => "GROWTH",
            ActionType::Sporer => "SPORER",
        }
    }

    pub fn get_index(&self) -> u8 {
        match self {
            ActionType::Wait => 0,
            ActionType::Growth => 1,
            ActionType::Sporer => 2,
        }
    }
}
