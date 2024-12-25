use crate::game_entities::{
    coord::{self, Coord},
    grid,
};

const MAX_NB_OF_BIT: usize = grid::MAX_HEIGHT * grid::MAX_WIDTH;

fn nb_bit_of_type(width: u8, height: u8) -> usize {
    (width as usize) * (height as usize)
}

fn nb_bit_per_segement(width: u8, height: u8) -> usize {
    128 - (128 % nb_bit_of_type(width, height))
}

fn nb_segement(width: u8, height: u8) -> usize {
    (MAX_NB_OF_BIT + (nb_bit_per_segement(width, height) - 1)) / nb_bit_per_segement(width, height)
}

const NB_BIT_PER_SEGEMENT: usize = 128 - (128 % grid::MAX_WIDTH);

const MAX_NB_SEGEMENTS: usize = (MAX_NB_OF_BIT + (NB_BIT_PER_SEGEMENT - 1)) / NB_BIT_PER_SEGEMENT;

#[derive(Debug, Clone, Copy)]
pub struct BinaryMap {
    bits: [u128; MAX_NB_SEGEMENTS], // Array of u128 to store  bits
    width: u8,
    height: u8,
    nb_segement: usize,
    nb_bit_per_segement: usize,
    bit_per_segement: usize,
}

fn get_mask_row(width: u8) -> u32 {
    (1 << width) - 1
}

impl BinaryMap {
    pub fn new(width: u8, height: u8) -> Self {
        BinaryMap {
            bits: [0; MAX_NB_SEGEMENTS],
            width,
            height,
            nb_segement: nb_segement(width, height),
            nb_bit_per_segement: nb_bit_per_segement(width, height),
            bit_per_segement: nb_bit_of_type(width, height),
        }
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        let (segment_index, bit_index) =
            (index / self.bit_per_segement, index % self.bit_per_segement);
        if value {
            self.bits[segment_index] |= 1 << bit_index;
        } else {
            self.bits[segment_index] &= !(1 << bit_index);
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let (segment_index, bit_index) =
            (index / self.bit_per_segement, index % self.bit_per_segement);
        (self.bits[segment_index] & (1 << bit_index)) != 0
    }

    pub fn clear(&mut self) {
        for i in 0..self.nb_segement {
            self.bits[i] = 0;
        }
    }

    pub fn get_row(&self, row: usize) -> u32 {
        let segment = (row * self.width as usize) / self.nb_bit_per_segement;
        (self.bits[segment] >> ((row * self.width as usize) % self.nb_bit_per_segement)) as u32
            & get_mask_row(self.width)
    }

    pub fn get_next_coord(&self, y_coord: usize, x_coord: usize) -> Option<Coord> {
        let current_row = self.get_row(y_coord);
        let adding = (current_row << 1) | (current_row >> 1) | current_row;
        let available_x = adding ^ current_row;

        for x in 0..self.width as usize {
            if available_x & (1 << x) != 0 {
                return Some(coord::new(x as u8, y_coord as u8));
            }
        }
        for y in 0..self.height as usize {
            if y > 0 && !self.get_bit((y - 1) * self.width as usize + x_coord) {
                return Some(coord::new(x_coord as u8, y as u8));
            }
            if (y < (self.height as usize - 1))
                && !self.get_bit((y + 1) * self.width as usize + x_coord)
            {
                return Some(coord::new(x_coord as u8, y as u8));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_bit_of_type() {
        let mut binary_map = BinaryMap::new(3, 3);
        binary_map.set_bit(0, true);
        binary_map.set_bit(1, true);
        binary_map.set_bit(2, true);
        binary_map.set_bit(3, true);
        binary_map.set_bit(4, true);
        binary_map.set_bit(5, true);
        binary_map.set_bit(6, true);
        binary_map.set_bit(7, true);
        binary_map.set_bit(8, true);
        assert_eq!(binary_map.get_bit(0), true);
        assert_eq!(binary_map.get_bit(1), true);
        assert_eq!(binary_map.get_bit(2), true);
        assert_eq!(binary_map.get_bit(3), true);
        assert_eq!(binary_map.get_bit(4), true);
        assert_eq!(binary_map.get_bit(5), true);
        assert_eq!(binary_map.get_bit(6), true);
        assert_eq!(binary_map.get_bit(7), true);
        assert_eq!(binary_map.get_bit(8), true);
    }

    #[test]
    fn test_binary_map() {
        let mut binary_map = BinaryMap::new(3, 3);
        binary_map.set_bit(0, true);
        binary_map.set_bit(1, true);
        binary_map.set_bit(2, true);
        binary_map.set_bit(3, true);
        binary_map.set_bit(4, true);
        binary_map.set_bit(5, true);
        binary_map.set_bit(6, true);
        binary_map.set_bit(7, true);
        binary_map.set_bit(8, true);
        assert_eq!(binary_map.get_row(0), 0b111);
        assert_eq!(binary_map.get_row(1), 0b111);
        assert_eq!(binary_map.get_row(2), 0b111);
        assert_eq!(binary_map.get_next_coord(0, 0), None);
        assert_eq!(binary_map.get_next_coord(0, 1), None);
        assert_eq!(binary_map.get_next_coord(0, 2), None);
        assert_eq!(binary_map.get_next_coord(1, 0), None);
        assert_eq!(binary_map.get_next_coord(1, 1), None);
        assert_eq!(binary_map.get_next_coord(1, 2), None);
        assert_eq!(binary_map.get_next_coord(2, 0), None);
        assert_eq!(binary_map.get_next_coord(2, 1), None);
        assert_eq!(binary_map.get_next_coord(2, 2), None);
    }
}
