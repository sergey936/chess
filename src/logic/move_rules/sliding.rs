use crate::logic::entities::board::Board;
use crate::logic::entities::position::{File, Move, Position, Rank};

pub fn calculate_sliding_moves(
    board: &Board,
    from: Position,
    directions: &[(i8, i8)],
) -> Vec<Move> {
    let mut result = Vec::new();

    for (df, dr) in directions {
        let mut file = from.file.index() as i8;
        let mut rank = from.rank.index() as i8;

        loop {
            file += df;
            rank += dr;

            if file < 0 || file > 7 || rank < 0 || rank > 7 {
                break;
            }

            let to = Position::new(
                File::try_from(file as usize).unwrap(),
                Rank::try_from(rank as usize).unwrap(),
            );

            match board.get_square(&to) {
                None => {
                    result.push(Move { from, to });
                }
                Some(square) => {
                    let target_square = board.get_square(&from).unwrap();

                    if square.piece.piece_color != target_square.piece.piece_color {
                        result.push(Move { from, to });
                    };

                    break;
                }
            }
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::logic::move_rules::test_utils::get_targets_from_moves;
    
    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
    use crate::logic::entities::position::{File, Position, Rank};

    const HORIZONTAL_DIRS: [(i8, i8); 2] = [(1, 0), (-1, 0)];
    const VERTICAL_DIRS: [(i8, i8); 2] = [(0, 1), (0, -1)];
    const DIAGONAL_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    const ALL_DIRS: [(i8, i8); 8] =  [
        (1,0), (-1,0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    #[test]
    fn test_calculate_vertical_moves() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Rook, PieceColor::White), from);

        let moves = calculate_sliding_moves(&board, from, &VERTICAL_DIRS);
        let targets = get_targets_from_moves(moves);

        let expected_ranks = [Rank::One, Rank::Two, Rank::Three, Rank::Four, Rank::Six, Rank::Seven, Rank::Eight];

        assert_eq!(expected_ranks.len(), targets.len());
        for rank in  expected_ranks {
            assert!(targets.contains(&Position::new(from.file, rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_calculate_horizontal_moves() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White), from);

        let moves = calculate_sliding_moves(&board, from, &HORIZONTAL_DIRS);
        let targets = get_targets_from_moves(moves);

        let expected_files = [File::A, File::B, File::C, File::D, File::F, File::G, File::H];

        assert_eq!(targets.len(), expected_files.len());
        for file in expected_files {
            assert!(targets.contains(&Position::new(file, from.rank)));
        }

        assert!(!targets.contains(&from));
    }

    #[test]
    fn test_calculate_diagonal_moves() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Rook, PieceColor::White), from);

        let moves = calculate_sliding_moves(&board, from, &DIAGONAL_DIRS);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
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
    fn test_calculate_all_directions_moves() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Queen, PieceColor::White), from);

        let moves = calculate_sliding_moves(&board, from, &ALL_DIRS);
        let targets = get_targets_from_moves(moves);

        let expected_positions = [
            // vertical
            (File::E, Rank::One), (File::E, Rank::Two), (File::E, Rank::Three), (File::E, Rank::Four),
            (File::E, Rank::Six), (File::E, Rank::Seven), (File::E, Rank::Eight),
            // horizontal
            (File::A, Rank::Five), (File::B, Rank::Five), (File::C, Rank::Five), (File::D, Rank::Five),
            (File::F, Rank::Five), (File::G, Rank::Five), (File::H, Rank::Five),
            // diagonal
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
    fn test_calculate_sliding_moves_blocked_by_own_pieces() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        let color = PieceColor::White;
        board.add_piece(Piece::new(PieceType::Queen, color), from);

        for (file, rank) in [
            (File::E, Rank::Six), (File::E, Rank::Four), (File::D, Rank::Five), (File::D, Rank::Six),
            (File::D, Rank::Four), (File::F, Rank::Four), (File::F, Rank::Five), (File::F, Rank::Six),
        ] {
            board.add_piece(Piece::new(PieceType::Pawn, color), Position::new(file, rank));
        };

        let moves = calculate_sliding_moves(&board, from, &ALL_DIRS);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_calculate_sliding_moves_blocked_by_enemy_pieces() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Queen, PieceColor::White), from);

        let enemy_pieces_position = [
            (File::E, Rank::Six), (File::E, Rank::Four), (File::D, Rank::Five), (File::D, Rank::Six),
            (File::D, Rank::Four), (File::F, Rank::Four), (File::F, Rank::Five), (File::F, Rank::Six),
        ];

        for (file, rank) in enemy_pieces_position {
            board.add_piece(Piece::new(PieceType::Pawn, PieceColor::Black), Position::new(file, rank));
        };

        let moves = calculate_sliding_moves(&board, from, &ALL_DIRS);
        let targets = get_targets_from_moves(moves);

        assert_eq!(targets.len(), enemy_pieces_position.len());
        for (file, rank) in enemy_pieces_position {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }
}
