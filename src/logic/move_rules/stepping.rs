use crate::logic::entities::board::Board;
use crate::logic::entities::position::{File, Rank, Position, Move};

pub fn calculate_stepping_moves(
    board: &Board,
    from: Position,
    deltas: &[(isize, isize)],
) -> Vec<Move> {
    let mut result = Vec::new();
    let from_square = match board.get_square(&from) {
        Some(s) => s,
        None => return result,
    };

    for (df, dr) in deltas {
        let file = from.file.index() as isize + df;
        let rank = from.rank.index() as isize + dr;

        if file < 0 || file > 7 || rank < 0 || rank > 7 {
            continue;
        }

        let to = Position::new(
            File::try_from(file as usize).unwrap(),
            Rank::try_from(rank as usize).unwrap(),
        );

        match board.get_square(&to) {
            None => {
                result.push(Move { from, to });
            }
            Some(target) => {
                if target.piece.piece_color != from_square.piece.piece_color {
                    result.push(Move { from, to });
                }
            }
        }
    }

    result
}
