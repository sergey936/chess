use std::convert::TryFrom;
use crate::logic::entities::piece::Piece;

#[derive(Copy, Clone, Debug)]
pub enum File {A, B, C, D, E, F, G, H}

impl File {
    fn index(&self) -> usize {
        *self as usize
    }
}
impl TryFrom<usize> for File {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Rank {
    One, Two, Three, Four, Five, Six, Seven, Eight
}

impl Rank {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<usize> for Rank {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Rank::One),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            _ => Err(()),
        }
    }
}


pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Self {file, rank}
    }
}

pub struct Square {
    pub piece: Piece,
    pub position: Position,
}

pub struct Board {
    pub squares: [[Option<Square>; 8]; 8],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: std::array::from_fn(|_| {
                std::array::from_fn(|_| None)
            }),
        }
    }

    pub fn show_board(&self) -> &[[Option<Square>; 8]; 8]{
        &self.squares
    }

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        let file = position.file.index();
        let rank = position.rank.index();

        self.squares[rank][file] = Some(Square { piece, position });
    }
}