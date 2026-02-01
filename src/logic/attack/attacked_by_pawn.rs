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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceType, PieceColor};
    use crate::logic::entities::position::{Position, File, Rank};

    #[test]
    fn test_is_square_attacked_by_white_pawn_corner_a2_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Pawn, PieceColor::White),
            Position::new(File::A, Rank::Two),
        );

        let expected_attacked_squares = [
            Position::new(File::B, Rank::Three),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_pawn(&board, pos, PieceColor::White),
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

                if is_attacked_by_pawn(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

    #[test]
    fn test_is_square_attacked_by_white_pawn_center_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Pawn, PieceColor::White),
            Position::new(File::D, Rank::Four),
        );

        let expected_attacked_squares = [
            Position::new(File::C, Rank::Five),
            Position::new(File::E, Rank::Five),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_pawn(&board, pos, PieceColor::White),
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

                if is_attacked_by_pawn(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 2);
    }

    #[test]
    fn test_is_square_attacked_by_black_pawn_corner_a7_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Pawn, PieceColor::Black),
            Position::new(File::A, Rank::Seven),
        );

        let expected_attacked_squares = [
            Position::new(File::B, Rank::Six),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_pawn(&board, pos, PieceColor::Black),
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

                if is_attacked_by_pawn(&board, pos, PieceColor::Black) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

    #[test]
    fn test_is_square_attacked_by_black_pawn_center_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Pawn, PieceColor::Black),
            Position::new(File::E, Rank::Five),
        );

        let expected_attacked_squares = [
            Position::new(File::D, Rank::Four),
            Position::new(File::F, Rank::Four),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_pawn(&board, pos, PieceColor::Black),
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

                if is_attacked_by_pawn(&board, pos, PieceColor::Black) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 2);
    }

}