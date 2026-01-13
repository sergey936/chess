use crate::logic::entities::board::Board;
use crate::logic::entities::position::{Move, Position};
use crate::logic::move_rules::sliding::calculate_sliding_moves;

pub fn moves(board: &Board, from: Position) -> Vec<Move> {
    const DIRECTIONS: [(i8, i8); 4] = [(1,1), (1,-1), (-1,1), (-1,-1)];

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
    fn test_bishop_moves_center() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            // down-left
            (File::D, Rank::Four), (File::C, Rank::Three), (File::B, Rank::Two), (File::A, Rank::One),
            // up-right
            (File::F, Rank::Six), (File::G, Rank::Seven), (File::H, Rank::Eight),
            // up-left
            (File::D, Rank::Six), (File::C, Rank::Seven), (File::B, Rank::Eight),
            // down-right
            (File::F, Rank::Four), (File::G, Rank::Three), (File::H, Rank::Two),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_bishop_moves_blocked_by_own_pieces() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        let color = PieceColor::White;
        board.add_piece(Piece::new(PieceType::Bishop, color), from);

        for (file, rank) in [
            (File::D, Rank::Four),
            (File::F, Rank::Six),
            (File::D, Rank::Six),
            (File::F, Rank::Four),
        ] {
            board.add_piece(Piece::new(PieceType::Pawn, color), Position::new(file, rank));
        }

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_bishop_moves_can_capture_enemy() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White), from);

        let enemy_positions = [
            (File::D, Rank::Four),
            (File::F, Rank::Six),
            (File::D, Rank::Six),
            (File::F, Rank::Four),
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
    fn test_bishop_moves_stops_at_first_piece() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White), from);

        board.add_piece(Piece::new(PieceType::Pawn, PieceColor::Black), Position::new(File::F, Rank::Six));
        board.add_piece(Piece::new(PieceType::Pawn, PieceColor::Black), Position::new(File::G, Rank::Seven));

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        assert!(targets.contains(&Position::new(File::F, Rank::Six)));
        assert!(!targets.contains(&Position::new(File::G, Rank::Seven)));
        assert!(!targets.contains(&Position::new(File::H, Rank::Eight)));
    }

    #[test]
    fn test_bishop_moves_from_corner() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::A, Rank::One);
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White), from);

        let moves = moves(&board, from);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            (File::B, Rank::Two),
            (File::C, Rank::Three),
            (File::D, Rank::Four),
            (File::E, Rank::Five),
            (File::F, Rank::Six),
            (File::G, Rank::Seven),
            (File::H, Rank::Eight),
        ];

        assert_eq!(targets.len(), expected_positions.len());
        for (file, rank) in expected_positions {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }
}