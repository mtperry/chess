#![allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;
use crate::board::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW
}

pub const NUM_DIRECTIONS: usize = 8;
pub const ALL_DIRECTIONS: [Direction; NUM_DIRECTIONS] = [
    Direction::N,
    Direction::S,
    Direction::E,
    Direction::W,
    Direction::NE,
    Direction::NW,
    Direction::SE,
    Direction::SW
];

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

pub const NUM_SQUARES: usize = 64;
pub const ALL_SQUARES: [SQ; NUM_SQUARES] = [
    SQ::A1, SQ::B1, SQ::C1, SQ::D1, SQ::E1, SQ::F1, SQ::G1, SQ::H1,
    SQ::A2, SQ::B2, SQ::C2, SQ::D2, SQ::E2, SQ::F2, SQ::G2, SQ::H2,
    SQ::A3, SQ::B3, SQ::C3, SQ::D3, SQ::E3, SQ::F3, SQ::G3, SQ::H3,
    SQ::A4, SQ::B4, SQ::C4, SQ::D4, SQ::E4, SQ::F4, SQ::G4, SQ::H4,
    SQ::A5, SQ::B5, SQ::C5, SQ::D5, SQ::E5, SQ::F5, SQ::G5, SQ::H5,
    SQ::A6, SQ::B6, SQ::C6, SQ::D6, SQ::E6, SQ::F6, SQ::G6, SQ::H6,
    SQ::A7, SQ::B7, SQ::C7, SQ::D7, SQ::E7, SQ::F7, SQ::G7, SQ::H7,
    SQ::A8, SQ::B8, SQ::C8, SQ::D8, SQ::E8, SQ::F8, SQ::G8, SQ::H8,
];
pub const SQUARE_STRINGS: [&'static str; NUM_SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

impl SQ {
    pub fn from_index(value: usize) -> Self {
        match value % NUM_SQUARES {
            0  => SQ::A1,  1 => SQ::B1,  2 => SQ::C1,  3 => SQ::D1,
            4  => SQ::E1,  5 => SQ::F1,  6 => SQ::G1,  7 => SQ::H1,
            8  => SQ::A2,  9 => SQ::B2, 10 => SQ::C2, 11 => SQ::D2,
           12  => SQ::E2, 13 => SQ::F2, 14 => SQ::G2, 15 => SQ::H2,
           16  => SQ::A3, 17 => SQ::B3, 18 => SQ::C3, 19 => SQ::D3,
           20  => SQ::E3, 21 => SQ::F3, 22 => SQ::G3, 23 => SQ::H3,
           24  => SQ::A4, 25 => SQ::B4, 26 => SQ::C4, 27 => SQ::D4,
           28  => SQ::E4, 29 => SQ::F4, 30 => SQ::G4, 31 => SQ::H4,
           32  => SQ::A5, 33 => SQ::B5, 34 => SQ::C5, 35 => SQ::D5,
           36  => SQ::E5, 37 => SQ::F5, 38 => SQ::G5, 39 => SQ::H5,
           40  => SQ::A6, 41 => SQ::B6, 42 => SQ::C6, 43 => SQ::D6,
           44  => SQ::E6, 45 => SQ::F6, 46 => SQ::G6, 47 => SQ::H6,
           48  => SQ::A7, 49 => SQ::B7, 50 => SQ::C7, 51 => SQ::D7,
           52  => SQ::E7, 53 => SQ::F7, 54 => SQ::G7, 55 => SQ::H7,
           56  => SQ::A8, 57 => SQ::B8, 58 => SQ::C8, 59 => SQ::D8,
           60  => SQ::E8, 61 => SQ::F8, 62 => SQ::G8, 63 => SQ::H8,
            _  => unreachable!(),
        }
    }

    pub fn new(f: File, r: Rank) -> SQ {
        let i = NUM_FILES * r.to_index() + f.to_index();
        SQ::from_index(i)
    }

    pub fn color(self) -> Color {
        Color::from_index(self.to_index() % 2)
    }

    pub fn rank(self) -> Rank {
        Rank::from_index(self.to_index() / NUM_RANKS)
    }

    pub fn file(self) -> File {
        File::from_index(self.to_index() % NUM_FILES)
    }

    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn shift(self, d: Direction) -> Option<SQ> {
        let mut new_rank: Option<Rank> = None;
        let mut new_file: Option<File> = None;

        match d {
            Direction::N  => new_rank = self.rank().up(),
            Direction::S  => new_rank = self.rank().down(),
            Direction::E  => new_file = self.file().right(),
            Direction::W  => new_file = self.file().left(),
            Direction::NE => {
                new_rank = self.rank().up();
                new_file = self.file().right();
            }
            Direction::NW => {
                new_rank = self.rank().up();
                new_file = self.file().left();
            }
            Direction::SE => {
                new_rank = self.rank().down();
                new_file = self.file().right();
            }
            Direction::SW => {
                new_rank = self.rank().down();
                new_file = self.file().left();
            }
        }

        if new_rank.is_none() || new_file.is_none() {
            return None;
        }

        Some(SQ::new(new_file?, new_rank?))
    }
}

impl Display for SQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SQUARE_STRINGS[self.to_index()])
    }
}

impl FromStr for SQ {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return Err(Error::InvalidSquare);
        }

        let file = match bytes[0] | 0x20 {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            b'd' => 3,
            b'e' => 4,
            b'f' => 5,
            b'g' => 6,
            b'h' => 7,
            _ => return Err(Error::InvalidSquare),
        };

        let rank = match bytes[1] {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            _ => return Err(Error::InvalidSquare),
        };

        Ok(ALL_SQUARES[NUM_FILES * rank + file])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_on_indices_less_than_num_squares_return_correct_sq() {
        for i in 0..NUM_SQUARES {
            assert_eq!(SQ::from_index(i), ALL_SQUARES[i]);
        }
    }

    #[test]
    fn from_index_on_indices_equal_or_greater_than_num_squares_wraps() {
        assert_eq!(SQ::from_index(NUM_SQUARES),     SQ::A1);
        assert_eq!(SQ::from_index(NUM_SQUARES + 1), SQ::B1);
    }

    #[test]
    fn from_str_accepts_all_squares_lowercase() {
        for sq in ALL_SQUARES {
            assert_eq!(SQ::from_str(SQUARE_STRINGS[sq.to_index()]), Ok(sq));
        }
    }

    #[test]
    fn from_str_accepts_all_squares_uppercase() {
        for sq in ALL_SQUARES {
            let s = String::from(SQUARE_STRINGS[sq.to_index()]).to_ascii_lowercase();
            assert_eq!(SQ::from_str(&s), Ok(sq));
        }
    }

    #[test]
    fn from_str_rejects_invalid_strings() {
        let invalid = [
            "", "a", "1a", "i1", "a9", "z0",
            "a10", "aa1", "A0", "H9",
        ];

        for s in invalid {
            assert!(SQ::from_str(s).is_err());
        }
    }

    #[test]
    fn new_on_all_rank_file_combos_is_correct() {
        for sq in ALL_SQUARES.iter().copied() {
            assert_eq!(sq, SQ::new(sq.file(), sq.rank()));
        }
    }

    #[test]
    fn color_on_all_squares_returns_correct_color() {
        let mut color = Color::Black;
        for sq in ALL_SQUARES.iter().copied() {
            assert_eq!(color, sq.color());
            color = !color;
        }
    }

    #[test]
    fn rank_on_all_squares_returns_correct_rank() {
        for rank in ALL_RANKS.iter().copied() {
            for file in ALL_FILES.iter().copied() {
                let sq = SQ::new(file, rank);
                assert_eq!(sq.rank(), rank);
            }
        }
    }

    #[test]
    fn to_file_on_all_squares_returns_correct_file() {
        for rank in ALL_RANKS.iter().copied() {
            for file in ALL_FILES.iter().copied() {
                let sq = SQ::new(file, rank);
                assert_eq!(sq.file(), file);
            }
        }
    }

    #[test]
    fn to_index_on_all_squares_returns_correct() {
        for (i, sq) in ALL_SQUARES.iter().copied().enumerate() {
            assert_eq!(sq.to_index(), i);
        }
    } 

    #[test]
    fn shift() {
        assert_eq!(SQ::E4.shift(Direction::N),  Some(SQ::E5));
        assert_eq!(SQ::E4.shift(Direction::S),  Some(SQ::E3));
        assert_eq!(SQ::E4.shift(Direction::E),  Some(SQ::F4));
        assert_eq!(SQ::E4.shift(Direction::W),  Some(SQ::D4));
        assert_eq!(SQ::E4.shift(Direction::NE), Some(SQ::F5));
        assert_eq!(SQ::E4.shift(Direction::NW), Some(SQ::D5));
        assert_eq!(SQ::E4.shift(Direction::SE), Some(SQ::F3));
        assert_eq!(SQ::E4.shift(Direction::SW), Some(SQ::D3));

        assert_eq!(SQ::A1.shift(Direction::S),  None);
        assert_eq!(SQ::A1.shift(Direction::W),  None);
        assert_eq!(SQ::H1.shift(Direction::S),  None);
        assert_eq!(SQ::H1.shift(Direction::E),  None);
        assert_eq!(SQ::A8.shift(Direction::N),  None);
        assert_eq!(SQ::A8.shift(Direction::W),  None);
        assert_eq!(SQ::H8.shift(Direction::N),  None);
        assert_eq!(SQ::H8.shift(Direction::E),  None);
    }
}