pub type Coord = u16;

const BITS_Y: Coord = 4;
const MASK_Y: Coord = 0b1111;
const MASK_X: Coord = 0b1_1111;

pub const MASK_COORD: Coord = 0b0000_0001_1111_1111;

const MAX_X: u8 = 26;
const MAX_Y: u8 = 13;

pub fn new(x: u8, y: u8) -> Coord {
    if x > MAX_X || y > MAX_Y {
        panic!("\x1b[31mInvalid coordinates ({}, {})\x1b[0m", x, y);
    }
    ((x as Coord) << BITS_Y) | y as Coord
}

pub fn x(coord: Coord) -> u8 {
    ((coord >> BITS_Y) & MASK_X) as u8
}

pub fn y(coord: Coord) -> u8 {
    (coord & MASK_Y) as u8
}

pub fn manhattan_distance(coord1: Coord, coord2: Coord) -> u8 {
    x(coord1).abs_diff(x(coord2)) + y(coord1).abs_diff(y(coord2))
}

pub fn is_adjacent(coord1: Coord, coord2: Coord) -> bool {
    manhattan_distance(coord1, coord2) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_coord_new_invalid_x() {
        new(27, 0);
    }

    #[test]
    #[should_panic]
    fn test_coord_new_invalid_y() {
        new(0, 14);
    }

    #[test]
    fn test_pos_new() {
        let pos = new(26, 13);
        assert_eq!(x(pos), 26);
        assert_eq!(y(pos), 13);
    }

    #[test]
    fn test_pos_manhattan_distance() {
        let pos1 = new(1, 1);
        let pos2 = new(4, 5);
        assert_eq!(manhattan_distance(pos1, pos2), 7);
    }

    #[test]
    fn test_pos_is_adjacent() {
        let pos1 = new(1, 1);
        let pos2 = new(1, 2);
        assert!(is_adjacent(pos1, pos2));
    }

    #[test]
    fn test_pos_is_not_adjacent() {
        let pos1 = new(1, 1);
        let pos2 = new(1, 3);
        assert!(!is_adjacent(pos1, pos2));
    }

    #[test]
    fn test_partial_eq() {
        let pos1 = new(1, 1);
        let pos2 = new(1, 1);
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_partial_not_eq() {
        let pos1 = new(1, 1);
        let pos2 = new(1, 2);
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn test_clone() {
        let pos1 = new(1, 1);
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }
}
