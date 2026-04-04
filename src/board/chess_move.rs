#[allow(dead_code)]

use crate::board::{
    square::SQ,
    piece::PieceType
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: SQ,
    to: SQ,
    promotion: Option<PieceType>
}

impl Move {
    pub fn new(from: SQ, to: SQ, promotion: Option<PieceType>) -> Self {
        Move { from, to, promotion }
    }

    pub fn origin_sq(self) -> SQ {
        self.from
    }

    pub fn target_sq(self) -> SQ {
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
    fn create_move_with_no_promotion_and_access_its_fields() {
        let chess_move = Move::new(SQ::E2, SQ::E4, None);
        assert_eq!(chess_move.origin_sq(), SQ::E2);
        assert_eq!(chess_move.target_sq(), SQ::E4);
        assert_eq!(chess_move.promotion(), None);
    }

    #[test]
    fn create_move_with_promotion_and_access_its_fields() {
        let chess_move = Move::new(SQ::A1, SQ::B2, Some(PieceType::Queen));
        assert_eq!(chess_move.origin_sq(), SQ::A1);
        assert_eq!(chess_move.target_sq(), SQ::B2);
        assert_eq!(chess_move.promotion(), Some(PieceType::Queen));

        let chess_move = Move::new(SQ::H7, SQ::H8, Some(PieceType::Knight));
        assert_eq!(chess_move.origin_sq(), SQ::H7);
        assert_eq!(chess_move.target_sq(), SQ::H8);
        assert_eq!(chess_move.promotion(), Some(PieceType::Knight));
    }
}