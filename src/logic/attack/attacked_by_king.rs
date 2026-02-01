use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub fn is_attacked_by_king(
    board: &Board,
    pos: Position,
    color: PieceColor,
) -> bool {
    const KING_DELTAS: [(i8, i8); 8] = [
        (1,1), (1,0), (1,-1),
        (0,1),        (0,-1),
        (-1,1), (-1,0), (-1,-1),
    ];

    for (df, dr) in KING_DELTAS {
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
            if square.piece.piece_type == PieceType::King
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
    use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
    use crate::logic::entities::position::{File, Position, Rank};

    #[test]
    fn test_is_square_attacked_by_king_corner_a1_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::King, PieceColor::White),
            Position::new(File::A, Rank::One),
        );

        let expected_attacked_squares = [
            Position::new(File::A, Rank::Two),
            Position::new(File::B, Rank::One),
            Position::new(File::B, Rank::Two),
        ];

        // 1. Все ожидаемые клетки атакуются
        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_king(&board, pos, PieceColor::White),
                true
            );
        }

        // 2. Ничего лишнего не атакуется
        let mut attacked_count = 0;

        for file in 0..=7 {
            for rank in 0..=7 {
                let pos = Position::new(
                    File::try_from(file).unwrap(),
                    Rank::try_from(rank).unwrap(),
                );

                if is_attacked_by_king(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

    #[test]
    fn test_is_square_attacked_by_king_edge_a4_exact_amount() {
        let mut board = Board::create_empty_board();
        board.add_piece(
            Piece::new(PieceType::King, PieceColor::White),
            Position::new(File::A, Rank::Four),
        );

        let expected_attacked_squares = [
            Position::new(File::A, Rank::Three),
            Position::new(File::A, Rank::Five),
            Position::new(File::B, Rank::Three),
            Position::new(File::B, Rank::Four),
            Position::new(File::B, Rank::Five),
        ];

        for pos in expected_attacked_squares {
            assert_eq!(
                is_attacked_by_king(&board, pos, PieceColor::White),
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

                if is_attacked_by_king(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

    #[test]
    fn test_is_square_attacked_by_king_center_exact_amount() {
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
                is_attacked_by_king(&board, pos, PieceColor::White),
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

                if is_attacked_by_king(&board, pos, PieceColor::White) {
                    attacked_count += 1;
                }
            }
        }

        assert_eq!(attacked_count, expected_attacked_squares.len());
    }

}