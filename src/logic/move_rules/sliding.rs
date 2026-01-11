use crate::logic::entities::board::Board;
use crate::logic::entities::position::{File, Move, Position, Rank};

pub fn calculate_sliding_moves(
    board: &Board,
    from: Position,
    directions: &[(isize, isize)],
) -> Vec<Move> {
    let mut result = Vec::new();

    for (df, dr) in directions {
        let mut file = from.file.index() as isize;
        let mut rank = from.rank.index() as isize;

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
    use crate::logic::entities::board::Board;
    use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
    use crate::logic::entities::position::{File, Move, Position, Rank};

    const HORIZONTAL_DIRS: [(isize, isize); 2] = [(1,0), (-1,0)];
    const VERTICAL_DIRS: [(isize, isize); 2] = [(0, 1), (0, -1)];
    const DIAGONAL_DIRS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    fn get_targets_from_moves(moves: Vec<Move>) -> Vec<Position> {
        moves.into_iter().map(|m| m.to).collect()
    }
    #[test]
    fn test_calculate_vertical_moves() {
        let mut board = Board::create_empty_board();
        let from = Position::new(File::E, Rank::Five);
        board.add_piece(Piece::new(PieceType::Rook, PieceColor::White), from);

        let moves = calculate_sliding_moves(&board, from, &VERTICAL_DIRS);
        let targets = get_targets_from_moves(moves);

        for rank in [Rank::One, Rank::Two, Rank::Three, Rank::Four, Rank::Six, Rank::Seven, Rank::Eight] {
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

        for file in [File::A, File::B, File::C, File::D, File::F, File::G, File::H] {
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

        for (file, rank) in [
            (File::D, Rank::Four), (File::C, Rank::Three), (File::B, Rank::Two), (File::A, Rank::One),
            (File::F, Rank::Six), (File::G, Rank::Seven), (File::H, Rank::Eight),
            (File::D, Rank::Six), (File::C, Rank::Seven), (File::B, Rank::Eight),
            (File::F, Rank::Four), (File::G, Rank::Three), (File::H, Rank::Two),
        ] {
            assert!(targets.contains(&Position::new(file, rank)));
        }

        assert!(!targets.contains(&from));
    }

}
