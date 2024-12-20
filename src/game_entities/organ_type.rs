#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganType {
    Root = 0b1111,
    Basic = 0b1000,
    Harvester = 0b0110,
    Sporer = 0b0011,
    Tentacle = 0b0101,
}

impl OrganType {
    pub fn from_str(s: &str) -> OrganType {
        match s {
            "ROOT" => OrganType::Root,
            "BASIC" => OrganType::Basic,
            "HARVESTER" => OrganType::Harvester,
            "SPORER" => OrganType::Sporer,
            "TENTACLE" => OrganType::Tentacle,
            _ => panic!("Invalid organ type {}", s),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            OrganType::Root => "ROOT",
            OrganType::Basic => "BASIC",
            OrganType::Harvester => "HARVESTER",
            OrganType::Sporer => "SPORER",
            OrganType::Tentacle => "TENTACLE",
        }
    }

    pub fn get_cost(&self) -> u32 {
        match self {
            OrganType::Root => 0,
            OrganType::Basic => 10,
            OrganType::Harvester => 20,
            OrganType::Sporer => 30,
            OrganType::Tentacle => 40,
        }
    }

    pub fn get_index(&self) -> u8 {
        match self {
            OrganType::Root => 0,
            OrganType::Basic => 1,
            OrganType::Harvester => 2,
            OrganType::Sporer => 3,
            OrganType::Tentacle => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organ_type_from_str() {
        assert_eq!(OrganType::from_str("ROOT"), OrganType::Root);
        assert_eq!(OrganType::from_str("BASIC"), OrganType::Basic);
        assert_eq!(OrganType::from_str("HARVESTER"), OrganType::Harvester);
        assert_eq!(OrganType::from_str("SPORER"), OrganType::Sporer);
        assert_eq!(OrganType::from_str("TENTACLE"), OrganType::Tentacle);
    }

    #[test]
    fn test_organ_type_to_str() {
        assert_eq!(OrganType::Root.to_str(), "ROOT");
        assert_eq!(OrganType::Basic.to_str(), "BASIC");
        assert_eq!(OrganType::Harvester.to_str(), "HARVESTER");
        assert_eq!(OrganType::Sporer.to_str(), "SPORER");
        assert_eq!(OrganType::Tentacle.to_str(), "TENTACLE");
    }
}
