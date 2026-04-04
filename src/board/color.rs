#[allow(dead_code)]

use std::ops::{Not, Index, IndexMut};
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub const COUNT: usize = 2;
    pub const VARIANTS: [Color; Color::COUNT] = [
        Color::White,
        Color::Black
    ];

    pub const fn from_u8(value: u8) -> Self {
        match value % Color::COUNT as u8 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!()
        }
    }

    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl<T> Index<Color> for [T] {
    type Output = T;

    fn index(&self, color: Color) -> &Self::Output {
        &self[color as usize]
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self[color as usize]
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
    fn from_u8() {
        assert_eq!(Color::from_u8(0), Color::White);
        assert_eq!(Color::from_u8(1), Color::Black);
        assert_eq!(Color::from_u8(2), Color::White);
        assert_eq!(Color::from_u8(3), Color::Black);
    }

    #[test]
    fn to_u8() {
        assert_eq!(Color::White.to_u8(), 0);
        assert_eq!(Color::Black.to_u8(), 1);
    }

    #[test]
    fn from_str_on_valid_input_returns_correct_color() {
        assert_eq!(Color::from_str("w").unwrap(), Color::White);
        assert_eq!(Color::from_str("W").unwrap(), Color::White);
        assert_eq!(Color::from_str("b").unwrap(), Color::Black);
        assert_eq!(Color::from_str("B").unwrap(), Color::Black);
    }

    #[test]
    fn from_str_on_invalid_input_returns_invalid_color() {
        assert_eq!(Color::from_str("x").err(), Some(Error::InvalidColor));
    }

    #[test]
    fn not_on_both_colors() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
}