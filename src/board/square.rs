#[allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;
use std::ops::{Index, IndexMut};

use crate::error::Error;
use crate::board::{Rank, File, Color, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SQ {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl SQ {
    pub const COUNT: usize = 64;

    pub const VARIANTS: [SQ; SQ::COUNT as usize] = [
        SQ::A1, SQ::B1, SQ::C1, SQ::D1, SQ::E1, SQ::F1, SQ::G1, SQ::H1,
        SQ::A2, SQ::B2, SQ::C2, SQ::D2, SQ::E2, SQ::F2, SQ::G2, SQ::H2,
        SQ::A3, SQ::B3, SQ::C3, SQ::D3, SQ::E3, SQ::F3, SQ::G3, SQ::H3,
        SQ::A4, SQ::B4, SQ::C4, SQ::D4, SQ::E4, SQ::F4, SQ::G4, SQ::H4,
        SQ::A5, SQ::B5, SQ::C5, SQ::D5, SQ::E5, SQ::F5, SQ::G5, SQ::H5,
        SQ::A6, SQ::B6, SQ::C6, SQ::D6, SQ::E6, SQ::F6, SQ::G6, SQ::H6,
        SQ::A7, SQ::B7, SQ::C7, SQ::D7, SQ::E7, SQ::F7, SQ::G7, SQ::H7,
        SQ::A8, SQ::B8, SQ::C8, SQ::D8, SQ::E8, SQ::F8, SQ::G8, SQ::H8,
    ];

    pub const STRINGS: [&'static str; SQ::COUNT as usize] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];

    pub const fn from_u8(value: u8) -> Self {
        debug_assert!(value < (SQ::COUNT as u8));
        SQ::VARIANTS[(value as usize) % SQ::COUNT]
    }

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_coords(f: File, r: Rank) -> SQ {
        SQ::from_u8((r.to_u8() * (File::COUNT as u8)) + f.to_u8())
    }

    pub const fn color(self) -> Color {
        Color::from_u8(!(((self.to_u8() >> 3) ^ (self.to_u8() & 1))))
    }

    pub const fn rank(self) -> Rank {
        Rank::from_u8(self.to_u8() / (File::COUNT as u8))
    }

    pub const fn file(self) -> File {
        File::from_u8(self.to_u8() % (File::COUNT as u8))
    }

    pub const fn forward(self, color: Color) -> Option<SQ> {
        match color {
            Color::White => self.offset(Direction::N.file_offset(), Direction::N.rank_offset()),
            Color::Black => self.offset(Direction::S.file_offset(), Direction::S.rank_offset())
        }
    }

    pub const fn backward(self, color: Color) -> Option<SQ> {
        match color {
            Color::White => self.offset(Direction::S.file_offset(), Direction::S.rank_offset()),
            Color::Black => self.offset(Direction::N.file_offset(), Direction::N.rank_offset())
        }
    }

    pub const fn offset(self, file_delta: i8, rank_delta: i8) -> Option<SQ> {
        let new_file_index = (self.file() as i8) + file_delta;
        let new_rank_index = (self.rank() as i8) + rank_delta;

        if new_file_index < 0 || new_file_index >= File::COUNT as i8 ||
           new_rank_index < 0 || new_rank_index >= Rank::COUNT as i8 {
            return None;
        }

        let file = File::from_u8(new_file_index as u8);
        let rank = Rank::from_u8(new_rank_index as u8);
        Some(SQ::from_coords(file, rank))
    }
}

impl From<u8> for SQ {
    fn from(value: u8) -> Self {
        SQ::from_u8(value)
    }
}

impl From<(File, Rank)> for SQ {
    fn from(coords: (File, Rank)) -> Self {
        SQ::from_coords(coords.0, coords.1)
    }
}

impl FromStr for SQ {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return Err(Error::InvalidSquare);
        }

        let Some(file) = File::from_char(bytes[0] as char) else {
            return Err(Error::InvalidSquare);
        };

        let Some(rank) = Rank::from_char(bytes[1] as char) else {
            return Err(Error::InvalidSquare);
        };

        Ok(SQ::from_coords(file, rank))
    }
}

impl Display for SQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SQ::STRINGS[*self])
    }
}

impl<T> Index<SQ> for [T] {
    type Output = T;

    fn index(&self, square: SQ) -> &Self::Output {
        &self[square as usize]
    }
}

impl<T> IndexMut<SQ> for [T] {
    fn index_mut(&mut self, square: SQ) -> &mut Self::Output {
        &mut self[square as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        for i in 0..SQ::COUNT {
            assert_eq!(SQ::from_u8(i as u8), SQ::VARIANTS[i]);
        }
    }

    #[test]
    #[should_panic]
    fn from_u8_invalid() {
        let _ = SQ::from_u8(SQ::COUNT as u8);
    }

    #[test]
    fn to_u8() {
        for i in 0..SQ::COUNT {
            assert_eq!(SQ::VARIANTS[i].to_u8(), i as u8);
        }
    }

    #[test]
    fn from_str_accepts_all_squares_lowercase() {
        assert_eq!(SQ::from_str("a1"), Ok(SQ::A1)); 
        assert_eq!(SQ::from_str("A1"), Ok(SQ::A1));
        for sq in SQ::VARIANTS.iter().copied() {
            assert_eq!(SQ::from_str(SQ::STRINGS[sq]), Ok(sq));
        }
    }

    #[test]
    fn from_str_accepts_all_squares_uppercase() {
        for sq in SQ::VARIANTS.iter().copied() {
            let s = String::from(SQ::STRINGS[sq]).to_ascii_uppercase();
            assert_eq!(SQ::from_str(&s), Ok(sq));
        }
    }

    #[test]
    fn from_str() {
        let invalid = [
            "", "a", "1a", "i1", "a9", "z0",
            "a10", "aa1", "A0", "H9",
        ];

        for s in invalid {
            assert!(SQ::from_str(s).is_err());
        }
    }

    #[test]
    fn from_coords() {
        for sq in SQ::VARIANTS.iter().copied() {
            assert_eq!(sq, SQ::from_coords(sq.file(), sq.rank()));
        }
    }

    #[test]
    fn color() {
        for file in File::VARIANTS.iter().copied() {
            let mut expected_color = if file.to_u8() % 2 == 0 { Color::Black } else { Color::White };

            for rank in Rank::VARIANTS.iter().copied() {
                assert_eq!(SQ::from_coords(file, rank).color(), expected_color);
                expected_color = !expected_color;
            }
        }
    }

    #[test]
    fn rank() {
        for rank in Rank::VARIANTS.iter().copied() {
            for file in File::VARIANTS.iter().copied() {
                let sq = {
                    let i = (File::COUNT as u8) * rank.to_u8() + file.to_u8();
                    SQ::from_u8(i)
                };
                assert_eq!(sq.rank(), rank);
            }
        }
    }

    #[test]
    fn file() {
        for rank in Rank::VARIANTS.iter().copied() {
            for file in File::VARIANTS.iter().copied() {
                let sq = {
                    let i = (File::COUNT as u8) * rank.to_u8() + file.to_u8();
                    SQ::from_u8(i)
                };
                assert_eq!(sq.file(), file);
            }
        }
    }

    #[test]
    fn offset() {
        assert_eq!(SQ::D4.offset(0, 0),   Some(SQ::D4));
        assert_eq!(SQ::A1.offset(8, 8),   Some(SQ::H8));
        assert_eq!(SQ::H8.offset(-8, -8), Some(SQ::A1));

        assert_eq!(SQ::A1.offset(-1, 0), None);
        assert_eq!(SQ::A1.offset(0, -1), None);
        assert_eq!(SQ::H8.offset(1, 0),  None);
        assert_eq!(SQ::H8.offset(0, 1),  None);
    }

    #[test]
    fn display() {
        for sq in SQ::VARIANTS.iter().copied() {
            assert_eq!(format!("{}", sq), SQ::STRINGS[sq]);
        }
    }
}