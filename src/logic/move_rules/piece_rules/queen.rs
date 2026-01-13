use crate::logic::entities::board::Board;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::sliding::calculate_sliding_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DIRECTIONS: [(i8, i8); 8] = [
        (1,1), (1,-1), (-1,1), (-1,-1),
        (1,0), (-1,0), (0,1), (0,-1),
    ];

    calculate_sliding_moves(board, from, &DIRECTIONS)
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::logic::move_rules::test_utils::get_targets_from_moves;

    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
    use crate::logic::entities::position::{File, Position, Rank};

    #[test]
    fn test_queen_moves_from_center() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Queen, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            // vertical
            (File::E, Rank::One), (File::E, Rank::Two), (File::E, Rank::Three), (File::E, Rank::Four),
            (File::E, Rank::Six), (File::E, Rank::Seven), (File::E, Rank::Eight),
            // horizontal
            (File::A, Rank::Five), (File::B, Rank::Five), (File::C, Rank::Five), (File::D, Rank::Five),
            (File::F, Rank::Five), (File::G, Rank::Five), (File::H, Rank::Five),
            // diagonals
            (File::D, Rank::Four), (File::C, Rank::Three), (File::B, Rank::Two), (File::A, Rank::One),
            (File::F, Rank::Six), (File::G, Rank::Seven), (File::H, Rank::Eight),
            (File::D, Rank::Six), (File::C, Rank::Seven), (File::B, Rank::Eight),
            (File::F, Rank::Four), (File::G, Rank::Three), (File::H, Rank::Two),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_queen_blocked_by_own_pieces() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        let color = PieceColor::White;
        board.add_piece(Piece::new(PieceType::Queen, color), from);

        let blocking_positions = [
            (File::E, Rank::Six), (File::E, Rank::Four),
            (File::D, Rank::Five), (File::F, Rank::Five),
            (File::D, Rank::Six), (File::D, Rank::Four),
            (File::F, Rank::Six), (File::F, Rank::Four),
        ];

        for (file, rank) in blocking_positions {
            board.add_piece(Piece::new(PieceType::Pawn, color), Position::new(file, rank));
        }

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_queen_can_capture_enemy_but_not_go_beyond() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Queen, PieceColor::White), from);

        let enemy_positions = [
            (File::E, Rank::Six),     // up
            (File::E, Rank::Four),    // down
            (File::D, Rank::Five),    // left
            (File::F, Rank::Five),    // right
            (File::D, Rank::Six),     // up-left
            (File::F, Rank::Six),     // up-right
            (File::D, Rank::Four),    // down-left
            (File::F, Rank::Four),    // down-right
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
    fn test_queen_from_corner() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::A, Rank::One);
        board.add_piece(Piece::new(PieceType::Queen, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            // vertical
            (File::A, Rank::Two), (File::A, Rank::Three), (File::A, Rank::Four),
            (File::A, Rank::Five), (File::A, Rank::Six), (File::A, Rank::Seven), (File::A, Rank::Eight),
            // horizontal
            (File::B, Rank::One), (File::C, Rank::One), (File::D, Rank::One),
            (File::E, Rank::One), (File::F, Rank::One), (File::G, Rank::One), (File::H, Rank::One),
            // diagonal
            (File::B, Rank::Two), (File::C, Rank::Three), (File::D, Rank::Four),
            (File::E, Rank::Five), (File::F, Rank::Six), (File::G, Rank::Seven), (File::H, Rank::Eight),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }
}