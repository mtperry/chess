use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::board::{File, Rank, SQ};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BB(pub u64);

impl BB {
    pub const fn from_sq(sq: SQ) -> Self {
        BB::SQUARES[sq as usize]
    }

    pub const fn from_file(file: File) -> Self {
        BB::FILES[file as usize]
    }

    pub const fn from_rank(rank: Rank) -> Self {
        BB::RANKS[rank as usize]
    }

    pub fn from_sqs(sqs: &[SQ]) -> Self {
        let mut bb = BB::EMPTY;
        for &sq in sqs {
            bb.0 |= 1u64 << sq.to_u8();
        }
        bb
    }

    pub const fn is_set(self, sq: SQ) -> bool {
        (self.0 & (1u64 << sq.to_u8())) != 0
    }

    pub fn are_set(self, sqs: &[SQ]) -> bool {
        for &sq in sqs {
            if !self.is_set(sq) {
                return false;
            }
        }
        true
    }

    pub const fn count(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub const fn lsb_sq(self) -> Option<SQ> {
        if self.0 == 0 {
            None
        } else {
            Some(SQ::from_u8(self.0.trailing_zeros() as u8))
        }
    }

    pub fn print(self) {
        for rank in Rank::VARIANTS.iter().rev().copied() {
            for file in File::VARIANTS.iter().copied() {
                let sq = SQ::from_coords(file, rank);
                if self.is_set(sq) {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            print!("\n");
        }
    }

    pub const EMPTY: BB = BB(0);
    pub const FULL:  BB = BB(u64::MAX);
    pub const EDGES: BB = BB(0xFF818181818181FF);
    pub const CORNERS: BB = BB(0x8100000000000081);
    pub const DARK_SQUARES:  BB = BB(0xAA55AA55AA55AA55);
    pub const LIGHT_SQUARES: BB = BB(0x55AA55AA55AA55AA);
    pub const FILES: [BB; File::COUNT] = {
        let mut files = [BB(0); File::COUNT];
        let mut i = 0;
        while i < File::COUNT {
            files[i] = BB(0x0101010101010101u64 << i);
            i += 1;
        }
        files
    };

    pub const RANKS: [BB; Rank::COUNT] = {
        let mut ranks = [BB(0); Rank::COUNT as usize];
        let mut i = 0;
        while i < Rank::COUNT as usize {
            ranks[i] = BB(0xFFu64 << (i * 8));
            i += 1;
        }
        ranks
    };

    pub const SQUARES: [BB; SQ::COUNT] = {
        let mut squares = [BB(0); SQ::COUNT as usize];
        let mut i = 0;
        while i < SQ::COUNT as usize {
            squares[i] = BB(1u64 << i);
            i += 1;
        }
        squares
    };
}

impl Iterator for BB {
    type Item = SQ;

    fn next(&mut self) -> Option<Self::Item> {
        if *self == BB::EMPTY {
            None
        } else {
            let temp_bb = *self;
            *self &= *self - BB(1);
            temp_bb.lsb_sq()
        }
    }
}

impl Add for BB {
    type Output = BB;
    fn add(self, rhs: BB) -> BB {
        BB(self.0.wrapping_add(rhs.0))
    }
}

impl AddAssign for BB {
    fn add_assign(&mut self, rhs: BB) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl Sub for BB {
    type Output = BB;
    fn sub(self, rhs: BB) -> BB {
        BB(self.0.wrapping_sub(rhs.0))
    }
}

impl SubAssign for BB {
    fn sub_assign(&mut self, rhs: BB) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}   

impl std::ops::Mul for BB {
    type Output = BB;
    fn mul(self, rhs: BB) -> BB {
        BB(self.0.wrapping_mul(rhs.0))
    }
}

impl std::ops::MulAssign<BB> for BB {
    fn mul_assign(&mut self, rhs: BB) {
        self.0 = self.0.wrapping_mul(rhs.0);
    }
}

impl std::ops::BitAnd for BB {
    type Output = BB;
    fn bitand(self, rhs: BB) -> BB {
        BB(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for BB {
    fn bitand_assign(&mut self, rhs: BB) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOr for BB {
    type Output = BB;
    fn bitor(self, rhs: BB) -> BB {
        BB(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for BB {
    fn bitor_assign(&mut self, rhs: BB) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXor for BB {
    type Output = BB;
    fn bitxor(self, rhs: BB) -> BB {
        BB(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for BB {
    fn bitxor_assign(&mut self, rhs: BB) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::Not for BB {
    type Output = BB;
    fn not(self) -> BB {
        BB(!self.0)
    }
}

impl std::ops::Shl<u8> for BB {
    type Output = BB;
    fn shl(self, rhs: u8) -> BB {
        BB(self.0 << rhs)
    }
}
    
impl std::ops::ShlAssign<u8> for BB {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

impl std::ops::Shr<u8> for BB {
    type Output = BB;
    fn shr(self, rhs: u8) -> BB {
        BB(self.0 >> rhs)
    }
}

impl std::ops::ShrAssign<u8> for BB {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_sq() {
        for sq in SQ::VARIANTS.iter().copied() {
            assert_eq!(BB::from_sq(sq), BB(1)<< sq.to_u8());
        }
    }

    #[test]
    fn from_sqs() {
        assert_eq!(BB::from_sqs(&[]), BB::EMPTY);
        assert_eq!(BB::from_sqs(&[SQ::A8]), BB(0x0000000000000080));
        assert_eq!(BB::from_sqs(&[SQ::A1, SQ::A1]), BB(0x0000000000000001));
        assert_eq!(BB::from_sqs(&[SQ::F6, SQ::C3, SQ::E5]), BB(0x0000000000280001));
        assert_eq!(BB::from_sqs(&[SQ::A7, SQ::G5, SQ::B4, SQ::H2]), BB(0x8080000000000010));
    }

    #[test]
    fn from_file() {
        for file in File::VARIANTS.iter().copied() {
            assert_eq!(BB::from_file(file), BB(0x0101010101010101u64 << file.to_u8()));
        }
    }

    #[test]
    fn from_rank() {
        for rank in Rank::VARIANTS.iter().copied() {
            assert_eq!(BB::from_rank(rank), BB(0xFFu64 << (rank.to_u8() * 8)));
        }
    }

    #[test]
    fn is_set() {
        let bb = BB::from_sqs(&[SQ::A2, SQ::G5, SQ::D6]);
        assert!(bb.is_set(SQ::A2));
        assert!(bb.is_set(SQ::G5));
        assert!(bb.is_set(SQ::D6));
        assert!(!bb.is_set(SQ::E4));
    }

    #[test]
    fn are_set() {
        assert!(BB::from_sqs(&[]).are_set(&[]));
        assert!(BB::from_sqs(&[SQ::F5]).are_set(&[SQ::F5]));
        assert!(BB::from_sqs(&[SQ::H5, SQ::E3]).are_set(&[SQ::H5, SQ::E3]));
        assert!(BB::from_sqs(&[SQ::A1, SQ::C3, SQ::E5]).are_set(&[SQ::A1, SQ::C3, SQ::E5]));
        assert!(BB::from_sqs(&[SQ::H1, SQ::C7, SQ::B5, SQ::D2]).are_set(&[SQ::H1, SQ::C7, SQ::B5, SQ::D2]));

    }

    #[test]
    fn lsb_sq() {
        assert_eq!(BB::from_sq(SQ::A3).lsb_sq(), Some(SQ::A3));
        assert_eq!(BB::from_sq(SQ::E8).lsb_sq(), Some(SQ::E8));
        assert_eq!(BB::EMPTY.lsb_sq(), None);
        assert_eq!(BB::FULL.lsb_sq(), Some(SQ::A1));
    }
    
    #[test]
    fn count() {
        let bb = BB::from_sqs(&[SQ::C3, SQ::E8, SQ::B2]);
        assert_eq!(bb.count(), 3);
        assert_eq!(BB::FULL.count(), 64);
        assert_eq!(BB::EMPTY.count(), 0);      
    }

    #[test]
    fn iterator() {
        let bb = BB::FULL;

        for (i, sq) in bb.into_iter().enumerate() {
            assert_eq!(sq, SQ::from_u8(i as u8));
        }
    }
}  