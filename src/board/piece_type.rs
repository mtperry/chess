#![allow(dead_code)]

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn from_index(value: usize) -> Self {
        match value % NUM_PIECE_TYPES {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!()
        }
    }

    pub fn try_from_index(value: usize) -> Result<Self, Error> {
        if value < NUM_PIECE_TYPES {
            Ok(PieceType::from_index(value))
        } else {
            Err(Error::InvalidPieceType)
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    } 
}

pub const NUM_PIECE_TYPES: usize = 6;
pub const ALL_PIECE_TYPES: [PieceType; NUM_PIECE_TYPES] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_index_on_all_indices_return_correct_piece_type() {
        for i in 0..NUM_PIECE_TYPES {
            assert_eq!(PieceType::from_index(i), ALL_PIECE_TYPES[i]);
        }
    }

    #[test]
    fn from_index_on_out_of_bounds_indices() {
        for i in NUM_PIECE_TYPES..100 {
            assert_eq!(PieceType::from_index(i), ALL_PIECE_TYPES[i % NUM_PIECE_TYPES]);
        }
    }

    #[test]
    fn try_from_index_on_all_valid_indices_return_correct_piece_type() {
        for i in 0..NUM_PIECE_TYPES {
            assert_eq!(PieceType::try_from_index(i).unwrap(), ALL_PIECE_TYPES[i]);
        }
    }

    #[test]
    fn try_from_index_on_out_of_bounds_indices_returns_invalid_piece_type() {
        assert_eq!(PieceType::try_from_index(NUM_PIECE_TYPES).err(), Some(Error::InvalidPieceType));
        assert_eq!(PieceType::try_from_index(NUM_PIECE_TYPES + 1).err(), Some(Error::InvalidPieceType));
    }

    #[test]
    fn to_index_on_all_piece_types() {
        for(i, piece_type) in ALL_PIECE_TYPES.iter().copied().enumerate() {
            assert_eq!(piece_type.to_index(), i);
        }
    }    
}