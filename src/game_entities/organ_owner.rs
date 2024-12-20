#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganeOwner {
    PlayerOwner = 1,
    EnemyOwner = 0,
    NotOrgan = -1,
}

impl OrganeOwner {
    pub fn from_i32(i: i32) -> Option<OrganeOwner> {
        match i {
            1 => Some(OrganeOwner::PlayerOwner),
            0 => Some(OrganeOwner::EnemyOwner),
            -1 => Some(OrganeOwner::NotOrgan),
            _ => panic!("Invalid owner {} for organ", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organ_owner_from_i32() {
        assert_eq!(OrganeOwner::from_i32(1), Some(OrganeOwner::PlayerOwner));
        assert_eq!(OrganeOwner::from_i32(0), Some(OrganeOwner::EnemyOwner));
        assert_eq!(OrganeOwner::from_i32(-1), Some(OrganeOwner::NotOrgan));
    }
}
