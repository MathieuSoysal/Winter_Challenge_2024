pub type Coord = u16;

pub fn new(x: u8, y: u8) -> Coord {
    (x as u16) << 8 | y as u16
}

pub fn x(coord: Coord) -> u8 {
    (coord >> 8) as u8
}

pub fn y(coord: Coord) -> u8 {
    (coord & 0xff) as u8
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
    fn test_pos_new() {
        let pos = new(1, 2);
        assert_eq!(x(pos), 1);
        assert_eq!(y(pos), 2);
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
