#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq)]
pub enum PieceColor {
    White,
    Black,
}


pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {piece_type, piece_color}
    }
}