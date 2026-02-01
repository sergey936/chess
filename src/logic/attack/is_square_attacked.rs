use crate::logic::attack::attacked_by_king::is_attacked_by_king;
use crate::logic::attack::attacked_by_pawn::is_attacked_by_pawn;
use crate::logic::attack::attacked_by_sliding::is_attacked_by_sliding;
use crate::logic::attack::attacking_by_knight::is_attacked_by_knight;
use crate::logic::entities::board::Board;
use crate::logic::entities::piece::PieceColor;
use crate::logic::entities::position::Position;

pub fn is_square_attacked(
    board: &Board,
    pos: Position,
    by_color: PieceColor,
) -> bool {
    if is_attacked_by_pawn(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_knight(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_sliding(board, pos, by_color) {
        return true;
    }
    
    if is_attacked_by_king(board, pos, by_color) {
        return true;
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
    fn test_is_square_attacked_by_pawn() {
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
                is_square_attacked(&board, pos, PieceColor::White),
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

                if is_square_attacked(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 2);
    }

    #[test]
    fn test_is_square_attacked_by_knight() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Knight, PieceColor::White),
            Position::new(File::E, Rank::Five),
        );

        let expected_attacked_squares = [
            Position::new(File::C, Rank::Four),
            Position::new(File::C, Rank::Six),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Seven),
            Position::new(File::F, Rank::Three),
            Position::new(File::F, Rank::Seven),
            Position::new(File::G, Rank::Four),
            Position::new(File::G, Rank::Six),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_square_attacked(&board, pos, PieceColor::White),
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

                if is_square_attacked(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 8);
    }

    #[test]
    fn test_is_square_attacked_by_rook() {
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

        for pos in expected_attacked_squares {
            assert_eq!(
                is_square_attacked(&board, pos, PieceColor::White),
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

                if is_square_attacked(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

    #[test]
    fn test_is_square_attacked_by_king() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::King, PieceColor::White),
            Position::new(File::D, Rank::Four),
        );

        let expected_attacked_squares = [
            Position::new(File::C, Rank::Three),
            Position::new(File::C, Rank::Four),
            Position::new(File::C, Rank::Five),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Five),
            Position::new(File::E, Rank::Three),
            Position::new(File::E, Rank::Four),
            Position::new(File::E, Rank::Five),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_square_attacked(&board, pos, PieceColor::White),
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

                if is_square_attacked(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, 8);
    }

    #[test]
    fn test_is_square_attacked_combined_figures() {
        let mut board = Board::create_empty_board();

        board.add_piece(
            Piece::new(PieceType::Pawn, PieceColor::White),
            Position::new(File::D, Rank::Four),
        );
        board.add_piece(
            Piece::new(PieceType::Knight, PieceColor::White),
            Position::new(File::B, Rank::One),
        );
        board.add_piece(
            Piece::new(PieceType::King, PieceColor::White),
            Position::new(File::H, Rank::Eight),
        );

        let expected_attacked_squares = [
            Position::new(File::C, Rank::Five),
            Position::new(File::E, Rank::Five),

            Position::new(File::A, Rank::Three),
            Position::new(File::C, Rank::Three),
            Position::new(File::D, Rank::Two),

            Position::new(File::G, Rank::Seven),
            Position::new(File::G, Rank::Eight),
            Position::new(File::H, Rank::Seven),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_square_attacked(&board, pos, PieceColor::White),
                true
            );
        }
    }

    #[test]
    fn test_is_square_attacked_empty_board() {
        let board = Board::create_empty_board();

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                assert_eq!(
                    is_square_attacked(&board, pos, PieceColor::White),
                    false
                );
            }
        }
    }
}