enum PieceTypes {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

enum PieceColor {
    White,
    Black,
}

pub struct Piece {
    piece_type: PieceTypes,
    piece_color: PieceColor,
}