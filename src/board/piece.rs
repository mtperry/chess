#![allow(dead_code)]

use std::fmt::Display;

use crate::board::color::Color;
use crate::board::piece_type::*;
use crate::error::Error;


#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: Color,
    piece_type: PieceType
}

impl Piece {
    pub fn try_from_char(c: char) -> Result<Self, Error> {
        match c {
            'p' => Ok(BLACK_PAWN),
            'n' => Ok(BLACK_KNIGHT),
            'b' => Ok(BLACK_BISHOP),
            'r' => Ok(BLACK_ROOK),
            'q' => Ok(BLACK_QUEEN),
            'k' => Ok(BLACK_KING),
            'P' => Ok(WHITE_PAWN),
            'N' => Ok(WHITE_KNIGHT),
            'B' => Ok(WHITE_BISHOP),
            'R' => Ok(WHITE_ROOK),
            'Q' => Ok(WHITE_QUEEN),
            'K' => Ok(WHITE_KING),
            _   => Err(Error::InvalidPieceType)
        }
    }    

    pub const fn new(color: Color, piece_type: PieceType) -> Self {
        Piece {
            color,
            piece_type
        }
    }

    pub fn piece_type(self) -> PieceType {
        self.piece_type
    }

    pub fn color(self) -> Color {
        self.color
    }

    pub fn to_char(self) -> char {
        match self.piece_type {
            PieceType::Pawn   => if self.color == Color::White {'P'} else {'p'},
            PieceType::Knight => if self.color == Color::White {'N'} else {'n'},
            PieceType::Bishop => if self.color == Color::White {'B'} else {'b'},
            PieceType::Rook   => if self.color == Color::White {'R'} else {'r'},
            PieceType::Queen  => if self.color == Color::White {'Q'} else {'q'},
            PieceType::King   => if self.color == Color::White {'K'} else {'k'},
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.piece_type == other.piece_type
    }
}

impl Eq for Piece {}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

pub const NUM_PIECES: usize = 12;
pub const ALL_PIECES: [Piece; NUM_PIECES] = [
    BLACK_PAWN,
    BLACK_KNIGHT,
    BLACK_BISHOP,
    BLACK_ROOK,
    BLACK_QUEEN,
    BLACK_KING,
    WHITE_PAWN,
    WHITE_KNIGHT,
    WHITE_BISHOP,
    WHITE_ROOK,
    WHITE_QUEEN,
    WHITE_KING,
];
pub const PIECE_CHARS: [char; NUM_PIECES] = [
    'p', 'n', 'b', 'r', 'q', 'k',
    'P', 'N', 'B', 'R', 'Q', 'K',
];

pub const BLACK_PAWN:   Piece = Piece::new(Color::Black, PieceType::Pawn  );
pub const BLACK_KNIGHT: Piece = Piece::new(Color::Black, PieceType::Knight);
pub const BLACK_BISHOP: Piece = Piece::new(Color::Black, PieceType::Bishop);
pub const BLACK_ROOK:   Piece = Piece::new(Color::Black, PieceType::Rook  );
pub const BLACK_QUEEN:  Piece = Piece::new(Color::Black, PieceType::Queen );
pub const BLACK_KING:   Piece = Piece::new(Color::Black, PieceType::King  );
pub const WHITE_PAWN:   Piece = Piece::new(Color::White, PieceType::Pawn  );
pub const WHITE_KNIGHT: Piece = Piece::new(Color::White, PieceType::Knight);
pub const WHITE_BISHOP: Piece = Piece::new(Color::White, PieceType::Bishop);
pub const WHITE_ROOK:   Piece = Piece::new(Color::White, PieceType::Rook  );
pub const WHITE_QUEEN:  Piece = Piece::new(Color::White, PieceType::Queen );
pub const WHITE_KING:   Piece = Piece::new(Color::White, PieceType::King  );


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_char_on_all_piece_chars_return_correct_piece() {
        for (i, c) in PIECE_CHARS.iter().copied().enumerate() {
            assert_eq!(Piece::try_from_char(c).unwrap(), ALL_PIECES[i]);
        }
    }

    #[test]
    fn try_from_char_on_invalid_piece_char_return_invalid_piece_type_error() {
        let invalid_chars = ['x', '1', '-', ' '];
        for c in invalid_chars {
            assert_eq!(Piece::try_from_char(c).unwrap_err(), Error::InvalidPieceType);
        }
    }

    #[test]
    fn piece_type_on_all_pieces_return_correct_piece_type() {
            assert_eq!(BLACK_PAWN.piece_type(),   PieceType::Pawn);
            assert_eq!(BLACK_KNIGHT.piece_type(), PieceType::Knight);
            assert_eq!(BLACK_BISHOP.piece_type(), PieceType::Bishop);
            assert_eq!(BLACK_ROOK.piece_type(),   PieceType::Rook);
            assert_eq!(BLACK_QUEEN.piece_type(),  PieceType::Queen);
            assert_eq!(BLACK_KING.piece_type(),   PieceType::King); 
            assert_eq!(WHITE_PAWN.piece_type(),   PieceType::Pawn);
            assert_eq!(WHITE_KNIGHT.piece_type(), PieceType::Knight);
            assert_eq!(WHITE_BISHOP.piece_type(), PieceType::Bishop);
            assert_eq!(WHITE_ROOK.piece_type(),   PieceType::Rook);
            assert_eq!(WHITE_QUEEN.piece_type(),  PieceType::Queen);
            assert_eq!(WHITE_KING.piece_type(),   PieceType::King);
    }

    #[test]
    fn color_on_all_pieces_return_correct_color() {
            assert_eq!(BLACK_PAWN.color(),   Color::Black);
            assert_eq!(BLACK_KNIGHT.color(), Color::Black);
            assert_eq!(BLACK_BISHOP.color(), Color::Black);
            assert_eq!(BLACK_ROOK.color(),   Color::Black); 
            assert_eq!(BLACK_QUEEN.color(),  Color::Black);
            assert_eq!(BLACK_KING.color(),   Color::Black);
            assert_eq!(WHITE_PAWN.color(),   Color::White);
            assert_eq!(WHITE_KNIGHT.color(), Color::White);
            assert_eq!(WHITE_BISHOP.color(), Color::White);
            assert_eq!(WHITE_ROOK.color(),   Color::White);
            assert_eq!(WHITE_QUEEN.color(),  Color::White);
            assert_eq!(WHITE_KING.color(),   Color::White);
    }

    #[test]
    fn to_char_on_all_pieces_return_correct_char() {
        let piece_chars = [
            (BLACK_PAWN,   'p'),
            (BLACK_KNIGHT, 'n'),
            (BLACK_BISHOP, 'b'),
            (BLACK_ROOK,   'r'),
            (BLACK_QUEEN,  'q'),
            (BLACK_KING,   'k'),
            (WHITE_PAWN,   'P'),
            (WHITE_KNIGHT, 'N'),
            (WHITE_BISHOP, 'B'),
            (WHITE_ROOK,   'R'),
            (WHITE_QUEEN,  'Q'),
            (WHITE_KING,   'K'),
        ];

        for (piece, expected_char) in piece_chars {
            assert_eq!(piece.to_char(), expected_char);
        }
    }

    #[test]
    fn piece_equality_works_correctly() {
        assert_eq!(BLACK_PAWN,   BLACK_PAWN);
        assert_eq!(BLACK_KNIGHT, BLACK_KNIGHT);
        assert_eq!(BLACK_BISHOP, BLACK_BISHOP);
        assert_eq!(BLACK_ROOK,   BLACK_ROOK);
        assert_eq!(BLACK_QUEEN,  BLACK_QUEEN);
        assert_eq!(BLACK_KING,   BLACK_KING);
        assert_eq!(WHITE_PAWN,   WHITE_PAWN);
        assert_eq!(WHITE_KNIGHT, WHITE_KNIGHT);
        assert_eq!(WHITE_BISHOP, WHITE_BISHOP);
        assert_eq!(WHITE_ROOK,   WHITE_ROOK);
        assert_eq!(WHITE_QUEEN,  WHITE_QUEEN);
        assert_eq!(WHITE_KING,   WHITE_KING);
    }

    #[test]
    fn display_on_all_pieces_return_correct_string() {
        for (i, c) in PIECE_CHARS.iter().copied().enumerate() {
            assert_eq!(c.to_string(), ALL_PIECES[i].to_string());
        }
    }
}