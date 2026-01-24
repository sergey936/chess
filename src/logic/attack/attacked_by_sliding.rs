use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub fn is_attacked_by_sliding(
    board: &Board,
    pos: Position,
    color: PieceColor,
) -> bool {
    const ROOK_DIRS: [(i8, i8); 4] = [(1,0), (-1,0), (0,1), (0,-1)];
    const BISHOP_DIRS: [(i8, i8); 4] = [(1,1), (1,-1), (-1,1), (-1,-1)];

    if scan_sliding(board, pos, color, &ROOK_DIRS, &[PieceType::Rook, PieceType::Queen]) {
        return true;
    }

    if scan_sliding(board, pos, color, &BISHOP_DIRS, &[PieceType::Bishop, PieceType::Queen]) {
        return true;
    }

    false
}
fn scan_sliding(
    board: &Board,
    pos: Position,
    color: PieceColor,
    dirs: &[(i8, i8)],
    allowed: &[PieceType],
) -> bool {
    for (df, dr) in dirs {
        let mut file = pos.file.index() as i8;
        let mut rank = pos.rank.index() as i8;

        loop {
            file += df;
            rank += dr;

            if file < 0 || file > 7 || rank < 0 || rank > 7 {
                break;
            }

            let p = Position::new(
                File::try_from(file as usize).unwrap(),
                Rank::try_from(rank as usize).unwrap(),
            );

            match board.get_square(&p) {
                None => continue,
                Some(square) => {
                    if square.piece.piece_color == color
                        && allowed.contains(&square.piece.piece_type)
                    {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    false
}
