#![allow(dead_code)]

use std::ops::Not;
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black
}

const NUM_COLORS: usize = 2;
const ALL_COLORS: [Color; NUM_COLORS] = [
    Color::White,
    Color::Black
];

impl Color {
    pub fn from_index(value: usize) -> Self {
        match value % NUM_COLORS {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!()
        }
    }

    pub fn try_from_index(value: usize) -> Result<Self, Error> {
        if value < NUM_COLORS {
            Ok(Color::from_index(value))
        } else {
            Err(Error::InvalidColor)
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" | "W" => Ok(Color::White),
            "b" | "B" => Ok(Color::Black),
            _ => Err(Error::InvalidColor)
        }
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_index_on_all_indices_less_than_num_colors_returns_correct_color() {
        for i in 0..NUM_COLORS {
            assert_eq!(Color::from_index(i), ALL_COLORS[i]);
        }
    }

    #[test]
    fn from_index_on_indices_greater_than_num_colors_wraps() {
        assert_eq!(Color::from_index(NUM_COLORS), Color::White);
        assert_eq!(Color::from_index(NUM_COLORS + 1), Color::Black);
    }

    #[test]
    fn try_from_index_on_indexes_less_than_num_colors_return_correct_color() {
        for i in 0..NUM_COLORS {
            assert_eq!(Color::try_from_index(i).unwrap(), Color::from_index(i));
        }
    }
    
    #[test]
    fn try_from_index_on_invalid_indices_returns_invalid_color() {
        assert_eq!(Color::try_from_index(NUM_COLORS).err(), Some(Error::InvalidColor));
        assert_eq!(Color::try_from_index(NUM_COLORS + 1).err(), Some(Error::InvalidColor));
    }

    #[test]
    fn to_index_on_both_colors() {
        for(i, color) in ALL_COLORS.iter().copied().enumerate() {
            assert_eq!(color.to_index(), i)
        }
    }

    #[test]
    fn not_on_both_colors() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
}