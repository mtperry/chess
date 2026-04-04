
use crate::board::magic;
#[allow(dead_code)]

use crate::board::{
    color::Color,
    file::File,
    rank::Rank,
    square::SQ,
    bitboard::BB,
    piece::PieceType,
};

pub fn pawn_attacks(sq: SQ, color: Color) -> BB {
    PAWN_ATTACKS[color][sq]
}

pub fn knight_attacks(sq: SQ) -> BB {
    KNIGHT_ATTACKS[sq]
}

pub fn bishop_attacks(sq: SQ, occupied: BB) -> BB {
    magic::bishop_attacks(sq, occupied)
}

pub fn rook_attacks(sq: SQ, occupied: BB) -> BB {
    magic::rook_attacks(sq, occupied)
}

pub fn queen_attacks(sq: SQ, occupied: BB) -> BB {
    bishop_attacks(sq, occupied) | rook_attacks(sq, occupied)
}

pub fn king_attacks(sq: SQ) -> BB {
    KING_ATTACKS[sq]
}

static PAWN_ATTACKS: [[BB; SQ::COUNT]; Color::COUNT] = {
    let mut attacks = [[BB(0); SQ::COUNT]; Color::COUNT];
    let mut sq_index = 0;
    while sq_index < SQ::COUNT as usize {
        let sq = SQ::from_u8(sq_index as u8);
        attacks[Color::White.to_u8() as usize][sq_index] = generate_pawn_attacks(sq, Color::White);
        attacks[Color::Black.to_u8() as usize][sq_index] = generate_pawn_attacks(sq, Color::Black);
        sq_index += 1;
    }
    attacks
};

static KNIGHT_ATTACKS: [BB; SQ::COUNT as usize] = {
    let mut attacks = [BB(0); SQ::COUNT as usize];
    let mut sq_index = 0;
    while sq_index < SQ::COUNT as usize {
        let sq = SQ::from_u8(sq_index as u8);
        attacks[sq_index] = generate_knight_attacks(sq);
        sq_index += 1;
    }
    attacks
};

static KING_ATTACKS: [BB; SQ::COUNT as usize] = {
    let mut attacks = [BB(0); SQ::COUNT as usize];
    let mut sq_index = 0;
    while sq_index < SQ::COUNT as usize {
        let sq = SQ::from_u8(sq_index as u8);
        attacks[sq_index] = generate_king_attacks(sq);
        sq_index += 1;
    }
    attacks
};

const fn generate_pawn_attacks(sq: SQ, color: Color) -> BB {
    let mut attacks = BB(0);
    
    match color {
        Color::White => {
            if sq.rank().to_u8() < Rank::Eighth.to_u8() {
                if sq.file().to_u8() > File::A.to_u8() {
                    attacks.0 |= 1 << (sq.to_u8() as i8 + 7) as u8;
                }

                if sq.file().to_u8() < File::H.to_u8() {
                    attacks.0 |= 1 << (sq.to_u8() as i8 + 9) as u8;
                }
            }
        },
        Color::Black => {
            if sq.rank().to_u8() > Rank::First.to_u8() {
                if sq.rank().to_u8() > Rank::Eighth.to_u8() {
                    if sq.file().to_u8() > File::A.to_u8() {
                        attacks.0 |= 1 << (sq.to_u8() as i8 - 9) as u8;
                    }

                    if sq.file().to_u8() < File::H.to_u8() {
                        attacks.0 |= 1 << (sq.to_u8() as i8 - 7) as u8;
                    }
                }
            }
        }
    }
    attacks
}

const fn generate_knight_attacks(sq: SQ) -> BB {
    let mut attacks = BB(0);
    let knight_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2)
    ];

    let mut i = 0;
    while i < knight_moves.len() {
        if let Some(sq) = sq.offset(knight_moves[i].0, knight_moves[i].1) {
            attacks.0 |= 1 << sq.to_u8();
        }
        i += 1;
    }

    attacks
}

const fn generate_queen_attacks(sq: SQ, occupied: BB) -> BB {
    BB(magic::generate_bishop_attacks(sq, occupied).0 | magic::generate_rook_attacks(sq, occupied).0)
}

const fn generate_king_attacks(sq: SQ) -> BB {
    let mut attacks = BB(0);
    let king_moves = [
        (1, 0), (1, 1), (0, 1), (-1, 1),
        (-1, 0), (-1, -1), (0, -1), (1, -1)
    ];

    let mut i = 0;
    while i < king_moves.len() {
        if let Some(sq) = sq.offset(king_moves[i].0, king_moves[i].1) {
            attacks.0 |= 1 << sq.to_u8();
        }
        i += 1;
    }

    attacks
}

mod tests {
    use crate::board::attacks;

    use super::*;

    #[test]
    fn pawn_attacks_test() {
        // Test white pawn attacks
        for sq in SQ::VARIANTS.iter().copied() {
            let mut attacks = BB::EMPTY;

            if sq.rank() != Rank::First && sq.rank() != Rank::Eighth {
                if let Some(sq1) = sq.offset(-1,1) {
                    attacks |= BB(1) << sq1.to_u8();
                }

                if let Some(sq2) = sq.offset(1,1) {
                    attacks |= BB(1) << sq2.to_u8();
                }
            }

            assert_eq!(pawn_attacks(sq, Color::White), attacks);
        }

        // Test black pawn attacks
        for sq in SQ::VARIANTS.iter().copied() {
            let mut attacks = BB::EMPTY;

            if sq.rank() != Rank::First && sq.rank() != Rank::Eighth {
                if let Some(sq1) = sq.offset(-1,1) {
                    attacks |= BB(1) << sq1.to_u8();
                }

                if let Some(sq2) = sq.offset(1,1) {
                    attacks |= BB(1) << sq2.to_u8();
                }
            }

            assert_eq!(pawn_attacks(sq, Color::Black), attacks);
        }
    }

    #[test]
    fn knight_attacks_test() {
        let knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ];

        for sq in SQ::VARIANTS.iter().copied() 
        {
            let mut attacks = BB::EMPTY;
            for (file_offset, rank_offset) in knight_moves.iter().copied() {
                if let Some(target_sq) = sq.offset(file_offset, rank_offset) {
                    attacks |= BB(1) << target_sq.to_u8();
                }
            }

            assert_eq!(knight_attacks(sq), attacks);
        }
    }

    #[test]
    fn bishop_attacks_test() {
        // This test is more complex due to the sliding nature of bishop attacks.
        // We will test a few specific squares with known attack patterns.

        let occupied = BB::from_sqs(&[SQ::D4]); // Place a piece on D4 to block attacks

        // Test attacks from A1
        let expected_attacks_a1 = BB::from_sqs(&[SQ::B2, SQ::C3]);
        assert_eq!(bishop_attacks(SQ::A1, occupied), expected_attacks_a1);

        // Test attacks from H8
        let expected_attacks_h8 = BB::from_sqs(&[SQ::G7, SQ::F6]);
        assert_eq!(bishop_attacks(SQ::H8, occupied), expected_attacks_h8);

        // Test attacks from D4 (should be blocked in all directions)
        let expected_attacks_d4 = BB::EMPTY;
        assert_eq!(bishop_attacks(SQ::D4, occupied), expected_attacks_d4);
    }

    #[test]
    fn rook_attacks_test() {
        let occupied = BB::from_sqs(&[SQ::A5]);

        // Test attacks from A1
        let expected_attacks_a1 = BB::from_sqs(&[SQ::A2, SQ::A3, SQ::A4, SQ::B1, SQ::C1]);
        assert_eq!(rook_attacks(SQ::A1, occupied), expected_attacks_a1);

        // Test attacks from H8
        let expected_attacks_h8 = BB::from_sqs(&[SQ::H7, SQ::H6, SQ::H5, SQ::G8, SQ::F8]);
        assert_eq!(rook_attacks(SQ::H8, occupied), expected_attacks_h8);

        // Test attacks from D4 (should be blocked in all directions)
        let expected_attacks_d4 = BB::EMPTY;
        assert_eq!(rook_attacks(SQ::D4, occupied), expected_attacks_d4);
    }

    #[test]
    fn queen_attacks_test() {
        let occupied = BB::from_sqs(&[SQ::D4]); 

        // Test attacks from A1
        let expected_attacks_a1 = BB::from_sqs(&[SQ::A2, SQ::A3, SQ::A4, SQ::B1, SQ::C1, SQ::B2, SQ::C3]);
        assert_eq!(queen_attacks(SQ::A1, occupied), expected_attacks_a1);

        // Test attacks from H8
        let expected_attacks_h8 = BB::from_sqs(&[SQ::H7, SQ::H6, SQ::H5, SQ::G8, SQ::F8, SQ::G7, SQ::F6]);
        assert_eq!(queen_attacks(SQ::H8, occupied), expected_attacks_h8);

        // Test attacks from D4 (should be blocked in all directions)
        let expected_attacks_d4 = BB::EMPTY;
        assert_eq!(queen_attacks(SQ::D4, occupied), expected_attacks_d4);
    }

    #[test]
    fn king_attacks_test() {
        let king_moves = [
            (1, 0), (1, 1), (0, 1), (-1, 1),
            (-1, 0), (-1, -1), (0, -1), (1, -1)
        ];  

        for sq in SQ::VARIANTS.iter().copied() {
            let mut attacks = BB::EMPTY;
            for (file_offset, rank_offset) in king_moves.iter().copied() {
                if let Some(target_sq) = sq.offset(file_offset, rank_offset) {
                    attacks |= BB(1) << target_sq.to_u8();
                }
            }

            assert_eq!(king_attacks(sq), attacks);
        }
    }
}