use super::coord::{self, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganDirection {
    North,
    West,
    South,
    East,
    X,
}

impl OrganDirection {
    pub fn from_str(s: &str) -> OrganDirection {
        match s {
            "NORTH" | "N" => OrganDirection::North,
            "WEST" | "W" => OrganDirection::West,
            "SOUTH" | "S" => OrganDirection::South,
            "EAST" | "E" => OrganDirection::East,
            "X" => OrganDirection::X,
            _ => panic!("Invalid direction {}", s),
        }
    }

    pub fn from_char(c: char) -> OrganDirection {
        match c {
            'N' => OrganDirection::North,
            'W' => OrganDirection::West,
            'S' => OrganDirection::South,
            'E' => OrganDirection::East,
            'X' => OrganDirection::X,
            _ => panic!("Invalid direction {}", c),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            OrganDirection::North => 'N',
            OrganDirection::West => 'W',
            OrganDirection::South => 'S',
            OrganDirection::East => 'E',
            OrganDirection::X => 'X',
        }
    }

    pub fn opposite(&self) -> OrganDirection {
        match self {
            OrganDirection::North => OrganDirection::South,
            OrganDirection::West => OrganDirection::East,
            OrganDirection::South => OrganDirection::North,
            OrganDirection::East => OrganDirection::West,
            OrganDirection::X => OrganDirection::X,
        }
    }

    pub fn turn_left(&self) -> OrganDirection {
        match self {
            OrganDirection::North => OrganDirection::West,
            OrganDirection::West => OrganDirection::South,
            OrganDirection::South => OrganDirection::East,
            OrganDirection::East => OrganDirection::North,
            OrganDirection::X => OrganDirection::X,
        }
    }

    pub fn turn_right(&self) -> OrganDirection {
        match self {
            OrganDirection::North => OrganDirection::East,
            OrganDirection::West => OrganDirection::North,
            OrganDirection::South => OrganDirection::West,
            OrganDirection::East => OrganDirection::South,
            OrganDirection::X => OrganDirection::X,
        }
    }

    pub fn move_pos(&self, pos: coord::Coord) -> coord::Coord {
        match self {
            OrganDirection::North => coord::new(coord::x(pos), coord::y(pos) - 1),
            OrganDirection::West => coord::new(coord::x(pos) - 1, coord::y(pos)),
            OrganDirection::South => coord::new(coord::x(pos), coord::y(pos) + 1),
            OrganDirection::East => coord::new(coord::x(pos) + 1, coord::y(pos)),
            OrganDirection::X => coord::new(coord::x(pos), coord::y(pos)),
        }
    }
}

pub fn found_next_direction(src: Coord, dst: Coord) -> OrganDirection {
    if coord::x(src) < coord::x(dst) {
        return OrganDirection::East;
    }
    if coord::x(src) > coord::x(dst) {
        return OrganDirection::West;
    }
    if coord::y(src) < coord::y(dst) {
        return OrganDirection::South;
    }
    OrganDirection::North
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organ_direction_from_str() {
        assert_eq!(OrganDirection::from_str("NORTH"), OrganDirection::North);
        assert_eq!(OrganDirection::from_str("WEST"), OrganDirection::West);
        assert_eq!(OrganDirection::from_str("SOUTH"), OrganDirection::South);
        assert_eq!(OrganDirection::from_str("EAST"), OrganDirection::East);
    }

    #[test]
    fn test_organ_direction_from_char() {
        assert_eq!(OrganDirection::from_char('N'), OrganDirection::North);
        assert_eq!(OrganDirection::from_char('W'), OrganDirection::West);
        assert_eq!(OrganDirection::from_char('S'), OrganDirection::South);
        assert_eq!(OrganDirection::from_char('E'), OrganDirection::East);
    }

    #[test]
    fn test_organ_direction_to_char() {
        assert_eq!(OrganDirection::North.to_char(), 'N');
        assert_eq!(OrganDirection::West.to_char(), 'W');
        assert_eq!(OrganDirection::South.to_char(), 'S');
        assert_eq!(OrganDirection::East.to_char(), 'E');
    }

    #[test]
    fn test_organ_direction_opposite() {
        assert_eq!(OrganDirection::North.opposite(), OrganDirection::South);
        assert_eq!(OrganDirection::West.opposite(), OrganDirection::East);
        assert_eq!(OrganDirection::South.opposite(), OrganDirection::North);
        assert_eq!(OrganDirection::East.opposite(), OrganDirection::West);
    }

    #[test]
    fn test_organ_direction_turn_left() {
        assert_eq!(OrganDirection::North.turn_left(), OrganDirection::West);
        assert_eq!(OrganDirection::West.turn_left(), OrganDirection::South);
        assert_eq!(OrganDirection::South.turn_left(), OrganDirection::East);
        assert_eq!(OrganDirection::East.turn_left(), OrganDirection::North);
    }

    #[test]
    fn test_organ_direction_turn_right() {
        assert_eq!(OrganDirection::North.turn_right(), OrganDirection::East);
        assert_eq!(OrganDirection::West.turn_right(), OrganDirection::North);
        assert_eq!(OrganDirection::South.turn_right(), OrganDirection::West);
        assert_eq!(OrganDirection::East.turn_right(), OrganDirection::South);
    }

    #[test]
    fn test_organ_direction_move_pos() {
        let pos = crate::game_entities::coord::new(1, 1);
        assert_eq!(
            OrganDirection::North.move_pos(pos),
            crate::game_entities::coord::new(1, 0)
        );
        assert_eq!(
            OrganDirection::West.move_pos(pos),
            crate::game_entities::coord::new(0, 1)
        );
        assert_eq!(
            OrganDirection::South.move_pos(pos),
            crate::game_entities::coord::new(1, 2)
        );
        assert_eq!(
            OrganDirection::East.move_pos(pos),
            crate::game_entities::coord::new(2, 1)
        );
    }

    #[test]
    fn test_found_next_direction() {
        let src = coord::new(1, 1);
        assert_eq!(
            found_next_direction(src, coord::new(1, 2)),
            OrganDirection::South
        );
        assert_eq!(
            found_next_direction(src, coord::new(1, 0)),
            OrganDirection::North
        );
        assert_eq!(
            found_next_direction(src, coord::new(0, 1)),
            OrganDirection::West
        );
        assert_eq!(
            found_next_direction(src, coord::new(2, 1)),
            OrganDirection::East
        );
    }
}
