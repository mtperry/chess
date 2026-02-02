#[allow(dead_code)]

use std::str::FromStr;

use crate::error::Error;

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
    pub fn from_index(value: usize) -> Self {
        match value % NUM_FILES {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!()
        }
    }

    pub fn try_from_index(value: usize) -> Option<Self> {
        if value < NUM_FILES {
            Some(File::from_index(value))
        } else {
            None
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn right(self) -> Option<File> {
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

    pub fn left(self) -> Option<File> {
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
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::InvalidFile);
        }

        match s {
            "a" | "A" => Ok(File::A),
            "b" | "B" => Ok(File::B),
            "c" | "C" => Ok(File::C),
            "d" | "D" => Ok(File::D),
            "e" | "E" => Ok(File::E),
            "f" | "F" => Ok(File::F),
            "g" | "G" => Ok(File::G),
            "h" | "H" => Ok(File::H),
            _ => Err(Error::InvalidFile)
        }
    }
}

pub const NUM_FILES: usize = 8;
pub const ALL_FILES: [File; NUM_FILES] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_index() {
        for i in 0..NUM_FILES {
            assert_eq!(File::from_index(i), ALL_FILES[i]);
        }
        assert_eq!(File::from_index(NUM_FILES), File::A);
    }

    #[test]
    fn try_from_index() {
        for i in 0..NUM_FILES {
            assert_eq!(File::try_from_index(i).unwrap(), ALL_FILES[i]);
        }
        assert_eq!(File::try_from_index(NUM_FILES), None);
    }

    #[test]
    fn to_index() {
        for(i, file) in ALL_FILES.iter().copied().enumerate() {
            assert_eq!(file.to_index(), i)
        }
    }
}