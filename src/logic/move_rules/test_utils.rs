use crate::logic::entities::position::{Move, Position};

#[cfg(test)]
pub fn get_targets_from_moves(moves: Vec<Move>) -> Vec<Position> {
    moves.into_iter().map(|m| m.to).collect()
}