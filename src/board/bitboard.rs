
use derive_more::{
    Mul, Add, Sub, BitAnd, BitOr, BitXor, Not, Shl, Shr,
    MulAssign, AddAssign, SubAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign
};

use crate::board::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Mul, Add, Sub, BitAnd, BitOr, BitXor, Not, Shl, Shr)]
#[derive(MulAssign, AddAssign, SubAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign)]
pub struct BB(pub u64);

impl BB {
    pub const EMPTY: BB = BB(0);
    pub const FULL:  BB = BB(u64::MAX);
    pub const EDGES: BB = BB(0xFF818181818181FF);
    pub const DARK_SQUARES:  BB = BB(0xAA55AA55AA55AA55);
    pub const LIGHT_SQUARES: BB = BB(0x55AA55AA55AA55AA);

    pub fn from_sq(sq: SQ) -> Self {
        BB(1u64 << sq.to_index())
    }

    pub fn with_set(mut self, sq: SQ) -> Self {
        self |= BB::from_sq(sq);
        self
    }

    pub fn with_clear(mut self, sq: SQ) -> Self {
        self &= !BB::from_sq(sq);
        self
    }

    pub fn is_set(self, sq: SQ) -> bool {
        (self & BB::from_sq(sq)) != BB::EMPTY
    }

    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn lsb_sq(self) -> Option<SQ> {
        if self == BB::EMPTY {
            None
        } else {
            Some(SQ::from_index(self.0.trailing_zeros() as usize))
        }
    }

}

impl Iterator for BB {
    type Item = SQ;

    fn next(&mut self) -> Option<Self::Item> {
        if *self == BB::EMPTY {
            None
        } else {
            let lsb_index = self.0.trailing_zeros() as usize;
            *self &= *self - BB(1);
            Some(SQ::from_index(lsb_index))
        }
    }
}

impl std::ops::Mul for BB {
    type Output = BB;
    fn mul(self, rhs: BB) -> BB {
        BB(self.0 * rhs.0)
    }
}

impl std::ops::MulAssign<BB> for BB {
    fn mul_assign(&mut self, rhs: BB) {
        self.0 *= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_set() {
        assert!(BB::EMPTY.with_set(SQ::C3).is_set(SQ::C3));
        assert_eq!(
            BB::EMPTY
                .with_set(SQ::E5)
                .with_set(SQ::A1), 
            BB::from_sq(SQ::E5) | BB::from_sq(SQ::A1)
        );
    }

    #[test]
    fn with_clear() {
        assert!(!BB::FULL.with_clear(SQ::D4).is_set(SQ::D4));
        assert_eq!(
            BB::FULL
                .with_clear(SQ::F4)
                .with_clear(SQ::H8), 
            BB::FULL & !BB::from_sq(SQ::F4) & !BB::from_sq(SQ::H8) 
        );
    }

    #[test]
    fn is_set() {
        let bb = BB::EMPTY
            .with_set(SQ::A2)
            .with_set(SQ::G5)
            .with_set(SQ::D6);
        assert!(bb.is_set(SQ::A2));
        assert!(bb.is_set(SQ::G5));
        assert!(bb.is_set(SQ::D6));
        assert!(!bb.is_set(SQ::E4));
    }

    #[test]
    fn lsb_sq() {
        assert_eq!(BB::from_sq(SQ::A3).lsb_sq(), Some(SQ::A3));
        assert_eq!(BB::from_sq(SQ::E8).lsb_sq(), Some(SQ::E8));
        assert_eq!(BB::EMPTY.lsb_sq(), None);
        assert_eq!(BB::FULL.lsb_sq(), Some(SQ::A1));
    }
    
    #[test]
    fn pop_count() {
        let bb = BB::EMPTY
            .with_set(SQ::B2)
            .with_set(SQ::C4)
            .with_set(SQ::F7);
        assert_eq!(bb.pop_count(), 3);
        assert_eq!(BB::FULL.pop_count(), 64);
        assert_eq!(BB::EMPTY.pop_count(), 0);      
    }

    #[test]
    fn iterator() {
        let bb = BB::FULL;

        for (i, sq) in bb.into_iter().enumerate() {
            assert_eq!(sq, SQ::from_index(i));
        }
    }
}  