use crate::logic::structures::piece::Piece;


enum File {A, B, C, D, E, F, G, H}

impl File {
    fn index(self) -> usize {
        self as usize
    }
}

enum Rank {One, Two, Three, Four, Five, Six, Seven, Eight}

impl Rank {
    fn index(self) -> usize {
        self as usize
    }
}

struct Position {
    file: File,
    rank: Rank,
}

struct Square {
    piece: Piece,
    position: Position,
}

pub struct Board {
    squares: [[Square; 8]; 8],
}

impl Board {
    
}