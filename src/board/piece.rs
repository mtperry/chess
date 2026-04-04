#[allow(dead_code)]

use std::ops::{Index, IndexMut};
use std::fmt::Display;

use crate::board::color::Color;
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
    pub fn to_u8(self) -> u8 {
        self as u8
    } 

    pub const COUNT: usize = 6;
    pub const VARIANTS: [PieceType; PieceType::COUNT] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];
}

impl<T> Index<PieceType> for [T] {
    type Output = T;

    fn index(&self, piece_type: PieceType) -> &Self::Output {
        &self[piece_type as usize]
    }
}

impl<T> IndexMut<PieceType> for [T] {
    fn index_mut(&mut self, piece_type: PieceType) -> &mut Self::Output {
        &mut self[piece_type as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType
}

impl Piece {
    pub fn try_from_char(c: char) -> Option<Piece> {
        match c {
            'p' => Some(Piece::BLACK_PAWN),
            'n' => Some(Piece::BLACK_KNIGHT),
            'b' => Some(Piece::BLACK_BISHOP),
            'r' => Some(Piece::BLACK_ROOK),
            'q' => Some(Piece::BLACK_QUEEN),
            'k' => Some(Piece::BLACK_KING),
            'P' => Some(Piece::WHITE_PAWN),
            'N' => Some(Piece::WHITE_KNIGHT),
            'B' => Some(Piece::WHITE_BISHOP),
            'R' => Some(Piece::WHITE_ROOK),
            'Q' => Some(Piece::WHITE_QUEEN),
            'K' => Some(Piece::WHITE_KING),
            _   => None
        }
    }    

    pub const fn new(color: Color, piece_type: PieceType) -> Self {
        Piece {
            color,
            piece_type
        }
    }

    pub fn type_of(self) -> PieceType {
        self.piece_type
    }

    pub fn color(self) -> Color {
        self.color
    }

    pub fn to_char(self) -> char {
        match self {
            Piece::BLACK_PAWN   => 'p',
            Piece::BLACK_KNIGHT => 'n',
            Piece::BLACK_BISHOP => 'b',
            Piece::BLACK_ROOK   => 'r',
            Piece::BLACK_QUEEN  => 'q',
            Piece::BLACK_KING   => 'k',
            Piece::WHITE_PAWN   => 'P',
            Piece::WHITE_KNIGHT => 'N',
            Piece::WHITE_BISHOP => 'B',
            Piece::WHITE_ROOK   => 'R',
            Piece::WHITE_QUEEN  => 'Q',
            Piece::WHITE_KING   => 'K'
        }
    }

    pub const COUNT: usize = 12;
    pub const VARIANTS: [Piece; Piece::COUNT] = [
        Piece::BLACK_PAWN,
        Piece::BLACK_KNIGHT,
        Piece::BLACK_BISHOP,
        Piece::BLACK_ROOK,
        Piece::BLACK_QUEEN,
        Piece::BLACK_KING,
        Piece::WHITE_PAWN,
        Piece::WHITE_KNIGHT,
        Piece::WHITE_BISHOP,
        Piece::WHITE_ROOK,
        Piece::WHITE_QUEEN,
        Piece::WHITE_KING,
    ];

    pub const CHARS: [char; Piece::COUNT] = [
        'p', 'n', 'b', 'r', 'q', 'k',
        'P', 'N', 'B', 'R', 'Q', 'K',
    ];

    pub const BLACK_PAWN:   Piece = Piece { color: Color::Black, piece_type: PieceType::Pawn   }; 
    pub const BLACK_KNIGHT: Piece = Piece { color: Color::Black, piece_type: PieceType::Knight };
    pub const BLACK_BISHOP: Piece = Piece { color: Color::Black, piece_type: PieceType::Bishop };
    pub const BLACK_ROOK:   Piece = Piece { color: Color::Black, piece_type: PieceType::Rook   };
    pub const BLACK_QUEEN:  Piece = Piece { color: Color::Black, piece_type: PieceType::Queen  };
    pub const BLACK_KING:   Piece = Piece { color: Color::Black, piece_type: PieceType::King   };
    pub const WHITE_PAWN:   Piece = Piece { color: Color::White, piece_type: PieceType::Pawn   };
    pub const WHITE_KNIGHT: Piece = Piece { color: Color::White, piece_type: PieceType::Knight };
    pub const WHITE_BISHOP: Piece = Piece { color: Color::White, piece_type: PieceType::Bishop };
    pub const WHITE_ROOK:   Piece = Piece { color: Color::White, piece_type: PieceType::Rook   };
    pub const WHITE_QUEEN:  Piece = Piece { color: Color::White, piece_type: PieceType::Queen  };
    pub const WHITE_KING:   Piece = Piece { color: Color::White, piece_type: PieceType::King   };
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PieceType tests
    #[test]
    fn to_index_on_all_piece_types() {
        for(i, piece_type) in PieceType::VARIANTS.iter().copied().enumerate() {
            assert_eq!(piece_type.to_u8(), i as u8);
        }
    }  

    // Piece tests
    #[test]
    fn try_from_char_on_all_piece_chars_return_correct_piece() {
        for (i, c) in Piece::CHARS.iter().copied().enumerate() {
            assert_eq!(Piece::try_from_char(c).unwrap(), Piece::VARIANTS[i]);
        }
    }

    #[test]
    fn try_from_char_on_invalid_piece_char_return_invalid_piece_type_error() {
        let invalid_chars = ['x', '1', '-', ' '];
        for c in invalid_chars {
            assert_eq!(Piece::try_from_char(c), None);
        }
    }

    #[test]
    fn type_of_on_all_pieces_return_correct_piece_type() {
            assert_eq!(Piece::BLACK_PAWN.type_of(),   PieceType::Pawn);
            assert_eq!(Piece::BLACK_KNIGHT.type_of(), PieceType::Knight);
            assert_eq!(Piece::BLACK_BISHOP.type_of(), PieceType::Bishop);
            assert_eq!(Piece::BLACK_ROOK.type_of(),   PieceType::Rook);
            assert_eq!(Piece::BLACK_QUEEN.type_of(),  PieceType::Queen);
            assert_eq!(Piece::BLACK_KING.type_of(),   PieceType::King); 
            assert_eq!(Piece::WHITE_PAWN.type_of(),   PieceType::Pawn);
            assert_eq!(Piece::WHITE_KNIGHT.type_of(), PieceType::Knight);
            assert_eq!(Piece::WHITE_BISHOP.type_of(), PieceType::Bishop);
            assert_eq!(Piece::WHITE_ROOK.type_of(),   PieceType::Rook);
            assert_eq!(Piece::WHITE_QUEEN.type_of(),  PieceType::Queen);
            assert_eq!(Piece::WHITE_KING.type_of(),   PieceType::King);
    }

    #[test]
    fn color_on_all_pieces_return_correct_color() {
            assert_eq!(Piece::BLACK_PAWN.color(),   Color::Black);
            assert_eq!(Piece::BLACK_KNIGHT.color(), Color::Black);
            assert_eq!(Piece::BLACK_BISHOP.color(), Color::Black);
            assert_eq!(Piece::BLACK_ROOK.color(),   Color::Black); 
            assert_eq!(Piece::BLACK_QUEEN.color(),  Color::Black);
            assert_eq!(Piece::BLACK_KING.color(),   Color::Black);
            assert_eq!(Piece::WHITE_PAWN.color(),   Color::White);
            assert_eq!(Piece::WHITE_KNIGHT.color(), Color::White);
            assert_eq!(Piece::WHITE_BISHOP.color(), Color::White);
            assert_eq!(Piece::WHITE_ROOK.color(),   Color::White);
            assert_eq!(Piece::WHITE_QUEEN.color(),  Color::White);
            assert_eq!(Piece::WHITE_KING.color(),   Color::White);
    }

    #[test]
    fn to_char_on_all_pieces_return_correct_char() {
        let piece_chars = [
            (Piece::BLACK_PAWN,   'p'),
            (Piece::BLACK_KNIGHT, 'n'),
            (Piece::BLACK_BISHOP, 'b'),
            (Piece::BLACK_ROOK,   'r'),
            (Piece::BLACK_QUEEN,  'q'),
            (Piece::BLACK_KING,   'k'),
            (Piece::WHITE_PAWN,   'P'),
            (Piece::WHITE_KNIGHT, 'N'),
            (Piece::WHITE_BISHOP, 'B'),
            (Piece::WHITE_ROOK,   'R'),
            (Piece::WHITE_QUEEN,  'Q'),
            (Piece::WHITE_KING,   'K'),
        ];

        for (piece, expected_char) in piece_chars {
            assert_eq!(piece.to_char(), expected_char);
        }
    }

    #[test]
    fn piece_equality_works_correctly() {
        assert_eq!(Piece::BLACK_PAWN,   Piece::BLACK_PAWN);
        assert_eq!(Piece::BLACK_KNIGHT, Piece::BLACK_KNIGHT);
        assert_eq!(Piece::BLACK_BISHOP, Piece::BLACK_BISHOP);
        assert_eq!(Piece::BLACK_ROOK,   Piece::BLACK_ROOK);
        assert_eq!(Piece::BLACK_QUEEN,  Piece::BLACK_QUEEN);
        assert_eq!(Piece::BLACK_KING,   Piece::BLACK_KING);
        assert_eq!(Piece::WHITE_PAWN,   Piece::WHITE_PAWN);
        assert_eq!(Piece::WHITE_KNIGHT, Piece::WHITE_KNIGHT);
        assert_eq!(Piece::WHITE_BISHOP, Piece::WHITE_BISHOP);
        assert_eq!(Piece::WHITE_ROOK,   Piece::WHITE_ROOK);
        assert_eq!(Piece::WHITE_QUEEN,  Piece::WHITE_QUEEN);
        assert_eq!(Piece::WHITE_KING,   Piece::WHITE_KING);
    }

    #[test]
    fn display_on_all_pieces_return_correct_string() {
        for (i, c) in Piece::CHARS.iter().copied().enumerate() {
            assert_eq!(c.to_string(), Piece::VARIANTS[i].to_string());
        }
    }
}