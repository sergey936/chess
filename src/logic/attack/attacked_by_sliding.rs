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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceType, PieceColor};
    use crate::logic::entities::position::{Position, File, Rank};

    #[test]
    fn test_is_square_attacked_by_rook_corner_a1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Rook, PieceColor::White),
            Position::new(File::A, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::A, Rank::Two),
            Position::new(File::A, Rank::Three),
            Position::new(File::A, Rank::Four),
            Position::new(File::A, Rank::Five),
            Position::new(File::A, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::A, Rank::Eight),

            Position::new(File::B, Rank::One),
            Position::new(File::C, Rank::One),
            Position::new(File::D, Rank::One),
            Position::new(File::E, Rank::One),
            Position::new(File::F, Rank::One),
            Position::new(File::G, Rank::One),
            Position::new(File::H, Rank::One),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_sliding(&board, position, PieceColor::White),
                true
            );
        }

        let mut attacked_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_sliding(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 14);
    }

    #[test]
    fn test_is_square_attacked_by_bishop_corner_a1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Bishop, PieceColor::White),
            Position::new(File::A, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::B, Rank::Two),
            Position::new(File::C, Rank::Three),
            Position::new(File::D, Rank::Four),
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_sliding(&board, position, PieceColor::White),
                true
            );
        }

        let mut attacked_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_sliding(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 7);
    }

    #[test]
    fn test_is_square_attacked_by_queen_corner_a1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Queen, PieceColor::White),
            Position::new(File::A, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::A, Rank::Two),
            Position::new(File::A, Rank::Three),
            Position::new(File::A, Rank::Four),
            Position::new(File::A, Rank::Five),
            Position::new(File::A, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::A, Rank::Eight),

            Position::new(File::B, Rank::One),
            Position::new(File::C, Rank::One),
            Position::new(File::D, Rank::One),
            Position::new(File::E, Rank::One),
            Position::new(File::F, Rank::One),
            Position::new(File::G, Rank::One),
            Position::new(File::H, Rank::One),

            Position::new(File::B, Rank::Two),
            Position::new(File::C, Rank::Three),
            Position::new(File::D, Rank::Four),
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_sliding(&board, position, PieceColor::White),
                true
            );
        }

        let mut attacked_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_sliding(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 21);
    }
}