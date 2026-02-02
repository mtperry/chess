#![allow(dead_code)]

use std::str::FromStr;

use crate::error::Error;

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

pub const NUM_RANKS: usize = 8;
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth
];

impl Rank {
    pub fn from_index(value: usize) -> Self {
        match value % NUM_RANKS {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!()
        }
    }

    pub fn try_from_index(value: usize) -> Result<Self, Error> {
        if value < NUM_RANKS {
            Ok(Rank::from_index(value))
        } else {
            Err(Error::InvalidRank)
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn up(self) -> Option<Rank> {
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

    pub fn down(self) -> Option<Rank> {
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
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Rank::First),
            "2" => Ok(Rank::Second),
            "3" => Ok(Rank::Third),
            "4" => Ok(Rank::Fourth),
            "5" => Ok(Rank::Fifth),
            "6" => Ok(Rank::Sixth),
            "7" => Ok(Rank::Seventh),
            "8" => Ok(Rank::Eighth),
            _   => Err(Error::InvalidRank)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_on_all_ranks_returns_correct_index() {
        for(i, rank) in ALL_RANKS.iter().copied().enumerate() {
            assert_eq!(rank.to_index(), i)
        }
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
    fn from_index_on_all_indices_less_than_num_ranks_returns_correct_rank() {
         for (i, rank) in ALL_RANKS.iter().copied().enumerate() {
            assert_eq!(rank.to_index(), i);
        }
    }

    #[test]
    fn from_index_on_indices_greater_than_num_ranks_wraps() {
        assert_eq!(Rank::from_index(8),  Rank::First);
        assert_eq!(Rank::from_index(9),  Rank::Second);
        assert_eq!(Rank::from_index(10), Rank::Third);
    }

    #[test]
    fn from_str_on_valid_input_returns_correct_rank() {
        assert_eq!(Rank::from_str("1").unwrap(), Rank::First);
        assert_eq!(Rank::from_str("2").unwrap(), Rank::Second);
        assert_eq!(Rank::from_str("3").unwrap(), Rank::Third);
        assert_eq!(Rank::from_str("4").unwrap(), Rank::Fourth);
        assert_eq!(Rank::from_str("5").unwrap(), Rank::Fifth);
        assert_eq!(Rank::from_str("6").unwrap(), Rank::Sixth);
        assert_eq!(Rank::from_str("7").unwrap(), Rank::Seventh);
        assert_eq!(Rank::from_str("8").unwrap(), Rank::Eighth);
    }

    #[test]
    fn from_str_on_invalid_input_returns_invalid_rank() {
        assert_eq!(Rank::from_str("9").err(), Some(Error::InvalidRank));
        assert_eq!(Rank::from_str("i").err(), Some(Error::InvalidRank));
        assert_eq!(Rank::from_str("$").err(), Some(Error::InvalidRank));
    }
}