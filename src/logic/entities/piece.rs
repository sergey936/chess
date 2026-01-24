#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> PieceColor {
        if *self == PieceColor::Black {
            PieceColor::White
        } else {
            PieceColor::Black
        }
    }
}


pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {piece_type, piece_color, has_moved: false}
    }
}