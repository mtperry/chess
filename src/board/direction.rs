
#[derive(Clone, Copy)]
pub enum Direction {
    N, NE, E, SE, S, SW, W, NW
}

impl Direction {
    pub const COUNT: u8 = 8;
    pub const VARIANTS: [Direction; Direction::COUNT as usize] = [
        Direction::N, Direction::NE, Direction::E, Direction::SE,
        Direction::S, Direction::SW, Direction::W, Direction::NW
    ];

    pub const fn rank_offset(&self) -> i8 {
        match self {
            Direction::N  => 1,
            Direction::NE => 1,
            Direction::E  => 0,
            Direction::SE => -1,
            Direction::S  => -1,
            Direction::SW => -1,
            Direction::W  => 0,
            Direction::NW => 1
        }
    }

    pub const fn file_offset(&self) -> i8 {
        match self {
            Direction::N  => 0,
            Direction::NE => 1,
            Direction::E  => 1,
            Direction::SE => 1,
            Direction::S  => 0,
            Direction::SW => -1,
            Direction::W  => -1,
            Direction::NW => -1
        }
    }
}