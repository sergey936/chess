use crate::logic::entities::board::Board;
use crate::logic::entities::piece::PieceType;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::piece_rules::{bishop, king, knight, pawn, queen, rook};

pub fn get_legal_moves(board: &Board, from: Position) -> Vec<Move> {
    let square = match board.get_square(&from) {
        Some(s) => s,
        None => return vec![],
    };

    match square.piece.piece_type {
        PieceType::Pawn => pawn::moves(board, from),
        PieceType::Rook => rook::moves(board, from),
        PieceType::Knight => knight::moves(board, from),
        PieceType::Bishop => bishop::moves(board, from),
        PieceType::Queen => queen::moves(board, from),
        PieceType::King => king::moves(board, from),
    }
}