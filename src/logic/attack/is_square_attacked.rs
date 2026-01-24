use crate::logic::attack::attacked_by_king::is_attacked_by_king;
use crate::logic::attack::attacked_by_pawn::is_attacked_by_pawn;
use crate::logic::attack::attacked_by_sliding::is_attacked_by_sliding;
use crate::logic::attack::attacking_by_knight::is_attacked_by_knight;
use crate::logic::entities::board::Board;
use crate::logic::entities::piece::PieceColor;
use crate::logic::entities::position::Position;

pub fn is_square_attacked(
    board: &Board,
    pos: Position,
    by_color: PieceColor,
) -> bool {
    if is_attacked_by_pawn(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_knight(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_sliding(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_king(board, pos, by_color) {
        return true;
    }

    false
}
