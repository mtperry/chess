pub mod direction;
pub mod color;
pub mod rank;
pub mod file;
pub mod square;
pub mod piece;
pub mod chess_move;
pub mod castle_rights;
pub mod bitboard;
pub mod attacks;
mod magic;

pub use direction::Direction;
pub use color::Color;
pub use rank::Rank;
pub use file::File;
pub use square::SQ;
pub use piece::{Piece, PieceType};
pub use chess_move::Move;
pub use castle_rights::CastleRights;
pub use bitboard::BB;
pub use attacks::{
    pawn_attacks, knight_attacks, bishop_attacks, rook_attacks, queen_attacks, king_attacks
};

pub enum FenError {
    InvalidTokenCount,
    InvalidRankCount,
    InvalidSideToMove,
    InvalidCastleRights,

}
pub struct Board {
    pieces: [Option<Piece>; SQ::COUNT],
    occupied_bbs: [BB; Color::COUNT],
    piece_type_bbs: [BB; PieceType::COUNT],
    castle_rights: [CastleRights; Color::COUNT],
    en_passant: Option<SQ>,
    halfmove_clock: u8,
    fullmove_number: u16
}

impl Board {

    pub fn occupied_bb(&self) -> BB {
        self.occupied_bbs[Color::White] | self.occupied_bbs[Color::Black]
    }

    pub fn color_bb(&self, color: Color) -> BB {
        self.occupied_bbs[color]
    }

    pub fn piece_bb(&self, piece: Piece) -> BB {
        self.color_bb(piece.color()) & self.piece_type_bbs[piece.type_of()]
    }

    pub fn piece_on(&self, sq: SQ) -> Option<Piece> {
        self.pieces[sq]
    }

    pub fn piece_count(&self, piece: Piece) -> u8 {
        self.piece_bb(piece).count()
    }
    fn add_piece(&mut self, sq: SQ, piece: Piece) {
        debug_assert_eq!(self.pieces[sq], None);
        self.pieces[sq] = Some(piece);
        self.occupied_bbs[piece.color()]     |= BB::from_sq(sq);
        self.piece_type_bbs[piece.type_of()] |= BB::from_sq(sq);
    }

    fn remove_piece(&mut self, sq: SQ) {
        if let Some(piece) = self.pieces[sq] {
            self.occupied_bbs[piece.color()]     &= !BB::from_sq(sq);
            self.piece_type_bbs[piece.type_of()] &= !BB::from_sq(sq);
            self.pieces[sq] = None;
        }
    }

    fn parse_fen_position(fen: &str, board: &mut Board) -> Result<(), FenError> {
        let parts: Vec<&str> = fen.split('/').collect();
        if parts.len() != Rank::COUNT {
            return Err(FenError::InvalidRankCount);
        }
        let rank = Rank::Eighth;
        loop {
            let mut file = File::A;
            for c in fen.chars() {
                if c == '/' {
                    if file != File::H {return Err(FenError::InvalidRankCount);}
                    rank = match rank.down() {
                        Some(r) => r,
                        None => return Err(Error::InvalidFen)
                    };
                    break;
                } else if c.is_digit(10) {
                    let Some(digit) = c.to_digit(10) else { 
                        return Err(FenError::InvalidFen); 
                    };

                    if digit == 0 || digit == 9 {
                        return Err(FenError::InvalidFen);
                    }

                    file = File::from_u8(file.to_u8() + digit as u8);
                } else {
                    let Some(piece) = Piece::from_char(c) else {
                        return Err(FenError::InvalidFileCount);
                    };
                    
                    board.add_piece(SQ::from_coords(file, rank), piece);
                    file = File::from_u8(file.to_u8() + 1);
                }
            }
        }

        Ok(())
    }
}



impl Default for Board {
    fn default() -> Self {
        Board {
            pieces: [None; SQ::COUNT],
            occupied_bbs: [BB(0); Color::COUNT],

            piece_type_bbs: [BB(0); PieceType::COUNT],
            castle_rights: [CastleRights::empty(); Color::COUNT],
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen() {}

    #[test]
    fn to_fen() {}

    #[test]
    fn piece_on() {}

    #[test]
    fn add_piece() {
        let mut board = Board::default();
        board.add_piece(SQ::E4, Piece::WHITE_KING);
        board.add_piece(SQ::H8, Piece::BLACK_QUEEN);
        assert_eq!(board.piece_on(SQ::E4), Some(Piece::WHITE_KING));
        assert_eq!(board.piece_on(SQ::H8), Some(Piece::BLACK_QUEEN));
    }

    #[test]
    fn remove_piece() {
        let mut board = Board::default();

        board.add_piece(sq, Piece::WHITE_KING);
        board.remove_piece(sq);
        assert_eq!(board.piece_on(sq), None);
        assert_eq!(board.color_bbs[Color::White], BB(0));
        assert_eq!(board.piece_type_bbs[PieceType::King] & BB::EMPTY.with_set(sq) != BB(0));
    }
}