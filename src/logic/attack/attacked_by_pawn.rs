use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub fn is_attacked_by_pawn(
    board: &Board,
    pos: Position,
    color: PieceColor,
) -> bool {
    let pawn_dirs: [(i8, i8); 2] = match color {
        PieceColor::White => [(-1, -1), (1, -1)],
        PieceColor::Black => [(-1, 1), (1, 1)],
    };

    for (df, dr) in pawn_dirs {
        let file = pos.file.index() as i8 + df;
        let rank = pos.rank.index() as i8 + dr;

        if file < 0 || file > 7 || rank < 0 || rank > 7 {
            continue;
        }

        let from = Position::new(
            File::try_from(file as usize).unwrap(),
            Rank::try_from(rank as usize).unwrap(),
        );

        if let Some(square) = board.get_square(&from) {
            if square.piece.piece_type == PieceType::Pawn
                && square.piece.piece_color == color
            {
                return true;
            }
        }
    }

    false
}