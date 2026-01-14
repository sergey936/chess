use crate::logic::entities::board::Board;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::stepping::calculate_stepping_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DELTAS: [(i8, i8); 8] = [
        (1,2), (2,1), (-1,2), (-2,1),
        (1,-2), (2,-1), (-1,-2), (-2,-1),
    ];

    calculate_stepping_moves(board, from, &DELTAS)
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::logic::move_rules::test_utils::get_targets_from_moves;

    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
    use crate::logic::entities::position::{File, Position, Rank};

    #[test]
    fn test_knight_moves_from_center() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            (File::D, Rank::Seven), (File::F, Rank::Seven),
            (File::C, Rank::Six),  (File::G, Rank::Six),
            (File::C, Rank::Four), (File::G, Rank::Four),
            (File::D, Rank::Three),(File::F, Rank::Three),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_knight_blocked_by_own_pieces() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        let color = PieceColor::White;
        board.add_piece(Piece::new(PieceType::Knight, color), from);

        let own_positions = [
            (File::D, Rank::Seven), (File::F, Rank::Seven),
            (File::C, Rank::Six),  (File::G, Rank::Six),
            (File::C, Rank::Four), (File::G, Rank::Four),
            (File::D, Rank::Three),(File::F, Rank::Three),
        ];

        for (file, rank) in own_positions {
            board.add_piece(Piece::new(PieceType::Pawn, color), Position::new(file, rank));
        }

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_knight_can_capture_enemy() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), from);

        let enemy_positions = [
            (File::D, Rank::Seven), (File::F, Rank::Seven),
            (File::C, Rank::Six),  (File::G, Rank::Six),
            (File::C, Rank::Four), (File::G, Rank::Four),
            (File::D, Rank::Three),(File::F, Rank::Three),
        ];

        for (file, rank) in enemy_positions {
            board.add_piece(Piece::new(PieceType::Pawn, PieceColor::Black), Position::new(file, rank));
        }

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), enemy_positions.len());
        for (file, rank) in enemy_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_knight_from_corner() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::A, Rank::One);
        board.add_piece(Piece::new(PieceType::Knight, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            (File::B, Rank::Three),
            (File::C, Rank::Two),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }
}
