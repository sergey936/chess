use crate::logic::entities::board::Board;
use crate::logic::entities::position::{File, Move, Position, Rank};

pub fn calculate_sliding_moves(
    board: &Board,
    from: Position,
    directions: &[(isize, isize)],
) -> Vec<Move> {
    let mut result = Vec::new();

    for (df, dr) in directions {
        let mut file = from.file.index() as isize;
        let mut rank = from.rank.index() as isize;

        loop {
            file += df;
            rank += dr;

            if file < 0 || file > 7 || rank < 0 || rank > 7 {
                break;
            }

            let to = Position::new(
                File::try_from(file as usize).unwrap(),
                Rank::try_from(rank as usize).unwrap(),
            );

            match board.get_square(&to) {
                None => {
                    result.push(Move { from, to });
                }
                Some(square) => {
                    let target_square = board.get_square(&from).unwrap();

                    if square.piece.piece_color != target_square.piece.piece_color {
                        result.push(Move { from, to });
                    };

                    break;
                }
            }
        }
    }

    result
}
