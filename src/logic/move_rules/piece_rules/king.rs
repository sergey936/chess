use crate::logic::entities::board::Board;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::stepping::calculate_stepping_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DELTAS: [(isize, isize); 8] = [
        (1,1), (1,0), (1,-1),
        (0,1),         (0,-1),
        (-1,1), (-1,0), (-1,-1),
    ];

    calculate_stepping_moves(board, from, &DELTAS)
}