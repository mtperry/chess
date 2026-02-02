#[allow(dead_code)]

use crate::board::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    from: SQ,
    to: SQ,
    promotion: Option<PieceType>
}

impl ChessMove {
    pub fn new(from: SQ, to: SQ, promotion: Option<PieceType>) -> Self {
        ChessMove { from, to, promotion }
    }

    pub fn from_sq(self) -> SQ {
        self.from
    }

    pub fn to_sq(self) -> SQ {
        self.to
    }

    pub fn promotion(self) -> Option<PieceType> {
        self.promotion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_move_with_no_promotion_and_access_its_fieldsa() {
        let chess_move = ChessMove::new(SQ::E2, SQ::E4, None);
        assert_eq!(chess_move.from_sq(), SQ::E2);
        assert_eq!(chess_move.to_sq(), SQ::E4);
        assert_eq!(chess_move.promotion(), None);
    }

    #[test]
    fn create_move_with_promotion_and_access_its_fields() {
        let chess_move = ChessMove::new(SQ::A1, SQ::B2, Some(PieceType::Queen));
        assert_eq!(chess_move.from_sq(), SQ::A1);
        assert_eq!(chess_move.to_sq(), SQ::B2);
        assert_eq!(chess_move.promotion(), Some(PieceType::Queen));

        let chess_move = ChessMove::new(SQ::H7, SQ::H8, Some(PieceType::Knight));
        assert_eq!(chess_move.from_sq(), SQ::H7);
        assert_eq!(chess_move.to_sq(), SQ::H8);
        assert_eq!(chess_move.promotion(), Some(PieceType::Knight));
    }
}