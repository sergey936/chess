use crate::logic::attack::is_square_attacked::is_square_attacked;
use crate::logic::entities::board::Board;
use crate::logic::entities::piece::PieceColor;
use crate::logic::entities::position::{File, Position, Rank};

fn can_castle_kingside(board: &Board, color: PieceColor) -> bool {
    let rank = match color {
        PieceColor::White => Rank::One,
        PieceColor::Black => Rank::Eight,
    };

    let king_pos = Position::new(File::E, rank);
    let rook_pos = Position::new(File::H, rank);

    let king = match board.get_square(&king_pos) {
        Some(s) => &s.piece,
        None => return false,
    };

    let rook = match board.get_square(&rook_pos) {
        Some(s) => &s.piece,
        None => return false,
    };

    if king.has_moved || rook.has_moved {
        return false;
    }

    for file in [File::F, File::G] {
        if board.get_square(&Position::new(file, rank)).is_some() {
            return false;
        }
    }

    for file in [File::E, File::F, File::G] {
        let pos = Position::new(file, rank);
        if is_square_attacked(board, pos, color.opposite()) {
            return false;
        }
    }

    true
}
