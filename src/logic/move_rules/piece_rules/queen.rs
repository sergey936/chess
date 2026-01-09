use crate::logic::entities::board::Board;
use crate::logic::entities::position::{File, Move, Position, Rank};
use crate::logic::move_rules::sliding::calculate_sliding_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (1,1), (1,-1), (-1,1), (-1,-1),
        (1,0), (-1,0), (0,1), (0,-1),
    ];

    calculate_sliding_moves(board, from, &DIRECTIONS)
}
