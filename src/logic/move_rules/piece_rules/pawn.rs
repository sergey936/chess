use crate::logic::entities::board::Board;
use crate::logic::entities::piece::PieceColor;
use crate::logic::entities::position::{File, Rank, Move, Position};

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    let mut result = Vec::new();
    let square = match board.get_square(&from) {
        Some(s) => s,
        None => return result,
    };

    let direction: isize = match square.piece.piece_color {
        PieceColor::White => 1,
        PieceColor::Black => -1,
    };

    let start_rank = match square.piece.piece_color {
        PieceColor::White => 1,
        PieceColor::Black => 6,
    };

    let file = from.file.index() as isize;
    let rank = from.rank.index() as isize;

    let one_forward = rank + direction;
    if one_forward >= 0 && one_forward <= 7 {
        let base_move_to = Position::new(
            File::try_from(file as usize).unwrap(),
            Rank::try_from(one_forward as usize).unwrap(),
        );

        if board.get_square(&base_move_to).is_none() {
            result.push(Move { from, to: base_move_to });

            if from.rank.index() == start_rank {
                let two_forward = rank + 2 * direction;
                let long_move_to = Position::new(
                    File::try_from(file as usize).unwrap(),
                    Rank::try_from(two_forward as usize).unwrap(),
                );

                if board.get_square(&long_move_to).is_none() {
                    result.push(Move { from, to: long_move_to });
                }
            }
        }
    }

    for df in [-1, 1] {
        let target_file = file + df;
        let target_rank = rank + direction;

        if target_file < 0 || target_file > 7 || target_rank < 0 || target_rank > 7 {
            continue;
        }

        let to = Position::new(
            File::try_from(target_file as usize).unwrap(),
            Rank::try_from(target_rank as usize).unwrap(),
        );

        if let Some(target) = board.get_square(&to) {
            if target.piece.piece_color != square.piece.piece_color {
                result.push(Move { from, to });
            }
        }
    }

    result
}
