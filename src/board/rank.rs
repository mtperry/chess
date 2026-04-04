#[allow(dead_code)]

use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::fmt::Display;

use crate::board::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

impl Rank {
    pub const COUNT: usize = 8;
    pub const VARIANTS: [Rank; Rank::COUNT] = [
        Rank::First,
        Rank::Second,
        Rank::Third,
        Rank::Fourth,
        Rank::Fifth,
        Rank::Sixth,
        Rank::Seventh,
        Rank::Eighth
    ];

    pub const fn from_u8(value: u8) -> Rank {
        debug_assert!(value < Rank::COUNT as u8);
        Rank::VARIANTS[(value as usize) % Rank::COUNT]
    }

    pub const fn from_char(c: char) -> Option<Rank> {
        match c {
            '1' => Some(Rank::First),
            '2' => Some(Rank::Second),
            '3' => Some(Rank::Third),
            '4' => Some(Rank::Fourth),
            '5' => Some(Rank::Fifth),
            '6' => Some(Rank::Sixth),
            '7' => Some(Rank::Seventh),
            '8' => Some(Rank::Eighth),
            _   => None
        }
    }

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn to_string(self) -> String {
        
    }

    pub const fn up(self) -> Option<Rank> {
        match self {
            Rank::First   => Some(Rank::Second),
            Rank::Second  => Some(Rank::Third),
            Rank::Third   => Some(Rank::Fourth),
            Rank::Fourth  => Some(Rank::Fifth),
            Rank::Fifth   => Some(Rank::Sixth),
            Rank::Sixth   => Some(Rank::Seventh),
            Rank::Seventh => Some(Rank::Eighth),
            Rank::Eighth  => None
        }
    }

    pub const fn down(self) -> Option<Rank> {
        match self {
            Rank::First   => None,
            Rank::Second  => Some(Rank::First),
            Rank::Third   => Some(Rank::Second),
            Rank::Fourth  => Some(Rank::Third),
            Rank::Fifth   => Some(Rank::Fourth),
            Rank::Sixth   => Some(Rank::Fifth),
            Rank::Seventh => Some(Rank::Sixth),
            Rank::Eighth  => Some(Rank::Seventh)
        }
    }

    pub const fn offset(self, delta: i8) -> Option<Rank> {
        let new_rank_index = (self.to_u8() as i8) + delta;
        if new_rank_index < 0 || new_rank_index >= Rank::COUNT as i8 {
            return None;
        }
        Some(Rank::from_u8(new_rank_index as u8))
    }
}

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        Rank::from_u8(value)
    }
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(());
        }
        let c = s.chars().next().unwrap();
        match c {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            _   => Err(())
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =match self {
            Rank::First   => "1".to_string(),
            Rank::Second  => "2".to_string(),
            Rank::Third   => "3".to_string(),
            Rank::Fourth  => "4".to_string(),
            Rank::Fifth   => "5".to_string(),
            Rank::Sixth   => "6".to_string(),
            Rank::Seventh => "7".to_string(),
            Rank::Eighth  => "8".to_string()
        };
        write!(f, "{}", string)
    }
}

impl <T> Index<Rank> for [T] {
    type Output = T;

    fn index(&self, rank: Rank) -> &Self::Output {
        &self[rank as usize]
    }
}

impl<T> IndexMut<Rank> for [T] {
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self[rank as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        for i in 0..Rank::COUNT {
            assert_eq!(Rank::from_u8(i as u8), Rank::VARIANTS[i]);
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    fn from_u8_invalid() {
        let _ = Rank::from_u8(Rank::COUNT as u8);
    }

    #[test]
    fn from_char() {
        assert_eq!(Rank::from_char('1').unwrap(), Rank::First);
        assert_eq!(Rank::from_char('2').unwrap(), Rank::Second);
        assert_eq!(Rank::from_char('3').unwrap(), Rank::Third);
        assert_eq!(Rank::from_char('4').unwrap(), Rank::Fourth);
        assert_eq!(Rank::from_char('5').unwrap(), Rank::Fifth);
        assert_eq!(Rank::from_char('6').unwrap(), Rank::Sixth);
        assert_eq!(Rank::from_char('7').unwrap(), Rank::Seventh);
        assert_eq!(Rank::from_char('8').unwrap(), Rank::Eighth);

        assert!(Rank::from_char('9').is_none());
        assert!(Rank::from_char('0').is_none());
        assert!(Rank::from_char('a').is_none());
        assert!(Rank::from_char('\0').is_none());
    }

    #[test]
    fn to_u8() {
        for i in 0..Rank::COUNT {
            assert_eq!(Rank::VARIANTS[i].to_u8(), i as u8);
        }
    }

    #[test]
    fn to_char() {
        assert_eq!(Rank::First.to_char(),   '1');
        assert_eq!(Rank::Second.to_char(),  '2');
        assert_eq!(Rank::Third.to_char(),   '3');
        assert_eq!(Rank::Fourth.to_char(),  '4');
        assert_eq!(Rank::Fifth.to_char(),   '5');
        assert_eq!(Rank::Sixth.to_char(),   '6');
        assert_eq!(Rank::Seventh.to_char(), '7');
        assert_eq!(Rank::Eighth.to_char(),  '8');
    }

    #[test]
    fn down() {
        assert_eq!(Rank::First.down(),   None);
        assert_eq!(Rank::Second.down(),  Some(Rank::First));
        assert_eq!(Rank::Third.down(),   Some(Rank::Second));
        assert_eq!(Rank::Fourth.down(),  Some(Rank::Third));
        assert_eq!(Rank::Fifth.down(),   Some(Rank::Fourth));
        assert_eq!(Rank::Sixth.down(),   Some(Rank::Fifth));
        assert_eq!(Rank::Seventh.down(), Some(Rank::Sixth));
        assert_eq!(Rank::Eighth.down(),  Some(Rank::Seventh));
    }

    #[test]
    fn up() {
        assert_eq!(Rank::First.up(),   Some(Rank::Second));
        assert_eq!(Rank::Second.up(),  Some(Rank::Third));
        assert_eq!(Rank::Third.up(),   Some(Rank::Fourth));
        assert_eq!(Rank::Fourth.up(),  Some(Rank::Fifth));
        assert_eq!(Rank::Fifth.up(),   Some(Rank::Sixth));
        assert_eq!(Rank::Sixth.up(),   Some(Rank::Seventh));
        assert_eq!(Rank::Seventh.up(), Some(Rank::Eighth));
        assert_eq!(Rank::Eighth.up(),  None);
    }
    
    #[test]
    fn offset() {
        assert_eq!(Rank::First.offset(1),    Some(Rank::Second));
        assert_eq!(Rank::Seventh.offset(-2), Some(Rank::Fifth));
        assert_eq!(Rank::Third.offset(-3),   None);
        assert_eq!(Rank::Fourth.offset(5),   None);
    }
    
    #[test]
    fn indexing() {
        assert_eq!(Rank::VARIANTS[Rank::First],   Rank::First);
        assert_eq!(Rank::VARIANTS[Rank::Second],  Rank::Second);
        assert_eq!(Rank::VARIANTS[Rank::Third],   Rank::Third);
        assert_eq!(Rank::VARIANTS[Rank::Fourth],  Rank::Fourth);
        assert_eq!(Rank::VARIANTS[Rank::Fifth],   Rank::Fifth);
        assert_eq!(Rank::VARIANTS[Rank::Sixth],   Rank::Sixth);
        assert_eq!(Rank::VARIANTS[Rank::Seventh], Rank::Seventh);
        assert_eq!(Rank::VARIANTS[Rank::Eighth],  Rank::Eighth);
    }
}