#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    InvalidColor,
    InvalidRank,
    InvalidFile,
    InvalidSquare,
    InvalidPieceType
}