use std::ffi::os_str::Display;
use std::str::FromStr;
use std::ops::{Index, IndexMut};

use crate::error::Error;
use crate::board::BB;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl File {
    pub const COUNT: usize = 8;
    pub const VARIANTS: [File; File::COUNT] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H
    ];

    pub const fn from_u8(value: u8) -> Self {
        debug_assert!(value < File::COUNT as u8);
        File::VARIANTS[value as usize]
    }

    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'a' | 'A' => Some(File::A),
            'b' | 'B' => Some(File::B),
            'c' | 'C' => Some(File::C),
            'd' | 'D' => Some(File::D),
            'e' | 'E' => Some(File::E),
            'f' | 'F' => Some(File::F),
            'g' | 'G' => Some(File::G),
            'h' | 'H' => Some(File::H),
            _ => None
        }
    }

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn right(self) -> Option<File> {
        match self {
            File::A => Some(File::B),
            File::B => Some(File::C),
            File::C => Some(File::D),
            File::D => Some(File::E),
            File::E => Some(File::F),
            File::F => Some(File::G),
            File::G => Some(File::H),
            File::H => None        
       }
    }

    pub const fn left(self) -> Option<File> {
        match self {
            File::A => None,
            File::B => Some(File::A),
            File::C => Some(File::B),
            File::D => Some(File::C),
            File::E => Some(File::D),
            File::F => Some(File::E),
            File::G => Some(File::F),
            File::H => Some(File::G)
        }
    }

    pub const fn offset(self, delta: i8) -> Option<File> {
        let new_file_index = (self.to_u8() as i8) + delta;
        if new_file_index < 0 || new_file_index >= File::COUNT as i8 {
            return None;
        }
        Some(File::from_u8(new_file_index as u8))
    }
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        File::from_u8(value)
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::InvalidFileCount);
        }
        let c = s.chars().next().unwrap();
        File::from_char(c).ok_or(Error::InvalidFile)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h'
        };
        write!(f, "{}", c)
    }
}
impl<T> Index<File> for [T] {
    type Output = T;

    fn index(&self, file: File) -> &Self::Output {
        &self[file as usize]
    }
}

impl<T> IndexMut<File> for [T] {
    fn index_mut(&mut self, file: File) -> &mut Self::Output {
        &mut self[file as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        for i in 0..File::COUNT {
            assert_eq!(File::from_u8(i as u8), File::VARIANTS[i]);
        }
    }

    #[test]
    #[should_panic]
    fn from_u8_invalid() {
        let _ = File::from_u8(File::COUNT as u8);
    }

    #[test]
    fn from_char() {
        assert_eq!(File::from_char('a').unwrap(), File::A);
        assert_eq!(File::from_char('A').unwrap(), File::A);
        assert_eq!(File::from_char('b').unwrap(), File::B);
        assert_eq!(File::from_char('B').unwrap(), File::B);
        assert_eq!(File::from_char('c').unwrap(), File::C);
        assert_eq!(File::from_char('C').unwrap(), File::C);
        assert_eq!(File::from_char('d').unwrap(), File::D);
        assert_eq!(File::from_char('D').unwrap(), File::D);
        assert_eq!(File::from_char('e').unwrap(), File::E);
        assert_eq!(File::from_char('E').unwrap(), File::E);
        assert_eq!(File::from_char('f').unwrap(), File::F);
        assert_eq!(File::from_char('F').unwrap(), File::F);
        assert_eq!(File::from_char('g').unwrap(), File::G);
        assert_eq!(File::from_char('G').unwrap(), File::G);
        assert_eq!(File::from_char('h').unwrap(), File::H);
        assert_eq!(File::from_char('H').unwrap(), File::H);

        assert!(File::from_char('i').is_none());
        assert!(File::from_char('I').is_none());
        assert!(File::from_char('\0').is_none());
    }

    #[test]
    fn to_u8() {
        for i in 0..File::COUNT {
            assert_eq!(File::VARIANTS[i as usize].to_u8(), i as u8);
        }
    }

    #[test]
    fn right() {
        assert_eq!(File::A.right(), Some(File::B));
        assert_eq!(File::B.right(), Some(File::C));
        assert_eq!(File::C.right(), Some(File::D));
        assert_eq!(File::D.right(), Some(File::E));
        assert_eq!(File::E.right(), Some(File::F));
        assert_eq!(File::F.right(), Some(File::G));
        assert_eq!(File::G.right(), Some(File::H));
        assert_eq!(File::H.right(), None);  
    }

    #[test]
    fn left() {
        assert_eq!(File::A.left(), None);
        assert_eq!(File::B.left(), Some(File::A));      
        assert_eq!(File::C.left(), Some(File::B));
        assert_eq!(File::D.left(), Some(File::C));
        assert_eq!(File::E.left(), Some(File::D));
        assert_eq!(File::F.left(), Some(File::E));
        assert_eq!(File::G.left(), Some(File::F));
        assert_eq!(File::H.left(), Some(File::G));
        assert_eq!(File::H.left(), Some(File::H));
    }

    #[test]
    fn offset() {
        assert_eq!(File::A.offset(5),  Some(File::E));
        assert_eq!(File::G.offset(-4), Some(File::C));
        assert_eq!(File::H.offset(2),  None);
        assert_eq!(File::B.offset(-2), None);
    }

    #[test]
    fn indexing() {
        assert_eq!(File::VARIANTS[File::A], File::A);
        assert_eq!(File::VARIANTS[File::B], File::B);
        assert_eq!(File::VARIANTS[File::C], File::C);
        assert_eq!(File::VARIANTS[File::D], File::D);
        assert_eq!(File::VARIANTS[File::E], File::E);
        assert_eq!(File::VARIANTS[File::F], File::F);
        assert_eq!(File::VARIANTS[File::G], File::G);
        assert_eq!(File::VARIANTS[File::H], File::H);
    }
}