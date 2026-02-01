use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub fn is_attacked_by_knight(
    board: &Board,
    pos: Position,
    by_color: PieceColor,
) -> bool {
    const KNIGHT_DELTAS: [(i8, i8); 8] = [
        (1,2), (2,1), (-1,2), (-2,1),
        (1,-2), (2,-1), (-1,-2), (-2,-1),
    ];

    for (df, dr) in KNIGHT_DELTAS {
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
            if square.piece.piece_type == PieceType::Knight
                && square.piece.piece_color == by_color
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
    fn test_is_square_attacked_by_knight_same_color() {
        let mut board = Board::create_empty_board();
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), Position::new(File::E, Rank::Five));

        for (row, col) in [(2, 3), (2, 5), (3, 2), (3, 6), (5, 2), (5, 6), (6, 3), (6, 5)] {
            let is_attacked = is_attacked_by_knight(
                &board,
                Position::new(
                    File::try_from(row as usize).unwrap(),
                    Rank::try_from(col as usize).unwrap()
                ),
                PieceColor::White,
            );

            assert_eq!(is_attacked, true);
        }
    }
    #[test]
    fn test_is_square_attacked_by_knight_other_color() {
        let mut board = Board::create_empty_board();
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), Position::new(File::E, Rank::Five));

        for (row, col) in [(2, 3), (2, 5), (3, 2), (3, 6), (5, 2), (5, 6), (6, 3), (6, 5)] {
            let is_attacked = is_attacked_by_knight(
                &board,
                Position::new(
                    File::try_from(row as usize).unwrap(),
                    Rank::try_from(col as usize).unwrap()
                ),
                PieceColor::Black,
            );

            assert_eq!(is_attacked, false);
        }
    }

    #[test]
    fn test_is_square_attacked_by_knight_corner_a8_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), Position::new(File::A, Rank::Eight));

        let expected_attacked_squares = [
            Position::new(File::B, Rank::Six),
            Position::new(File::C, Rank::Seven),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_knight(&board, position, PieceColor::White),
                true
            );
        }

        let mut attackers_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_knight(&board, pos, PieceColor::White) {
                    attackers_count += 1;
                }
            }
        }

        assert_eq!(attackers_count, 2);
    }
    #[test]
    fn test_is_square_attacked_by_knight_corner_a1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Knight, PieceColor::White),
            Position::new(File::A, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::B, Rank::Three),
            Position::new(File::C, Rank::Two),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_knight(&board, position, PieceColor::White),
                true
            );
        }

        let mut attackers_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_knight(&board, pos, PieceColor::White) {
                    attackers_count += 1;
                }
            }
        }

        assert_eq!(attackers_count, 2);
    }

    #[test]
    fn test_is_square_attacked_by_knight_corner_h1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Knight, PieceColor::White),
            Position::new(File::H, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::Three),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_knight(&board, position, PieceColor::White),
                true
            );
        }

        let mut attackers_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_knight(&board, pos, PieceColor::White) {
                    attackers_count += 1;
                }
            }
        }

        assert_eq!(attackers_count, 2);
    }

    #[test]
    fn test_is_square_attacked_by_knight_corner_h8_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::Knight, PieceColor::White),
            Position::new(File::H, Rank::Eight),
        );

        let expected_attacked_squares = [
            Position::new(File::F, Rank::Seven),
            Position::new(File::G, Rank::Six),
        ];

        for position in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_knight(&board, position, PieceColor::White),
                true
            );
        }

        let mut attackers_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_knight(&board, pos, PieceColor::White) {
                    attackers_count += 1;
                }
            }
        }

        assert_eq!(attackers_count, 2);
    }
}