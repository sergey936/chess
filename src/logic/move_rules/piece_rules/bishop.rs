use crate::logic::entities::board::Board;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::sliding::calculate_sliding_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DIRECTIONS: [(isize, isize); 4] = [(1,1), (1,-1), (-1,1), (-1,-1)];

    calculate_sliding_moves(board, from, &DIRECTIONS)
}
