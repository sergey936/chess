use crate::logic::board::builders::base_builder::BoardBuilder;
use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub struct ClassicChessBoardBuilder;

impl BoardBuilder for ClassicChessBoardBuilder {
    fn build(&self) -> Board {
        let mut board = Board::create_empty_board();

        let back_line = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (i, kind) in back_line.iter().enumerate() {
            board.add_piece(
                Piece::new(*kind, PieceColor::White),
                Position::new(File::try_from(i).unwrap(), Rank::One),
            );
            board.add_piece(
                Piece::new(PieceType::Pawn, PieceColor::White),
                Position::new(File::try_from(i).unwrap(), Rank::Two),
            );

            board.add_piece(
                Piece::new(*kind, PieceColor::Black),
                Position::new(File::try_from(i).unwrap(), Rank::Eight),
            );
            board.add_piece(
                Piece::new(PieceType::Pawn, PieceColor::Black),
                Position::new(File::try_from(i).unwrap(), Rank::Seven),
            );
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_chess_board_builder() {
        let builder = ClassicChessBoardBuilder;
        let board = builder.build();

        let back_line = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (i, expected_piece) in back_line.iter().enumerate() {
            let file = File::try_from(i).unwrap();

            let square = board
                .get_square(&Position::new(file, Rank::One))
                .expect("white back line piece must exist");

            assert_eq!(square.piece.piece_type, *expected_piece);
            assert_eq!(square.piece.piece_color, PieceColor::White);

            let pawn_square = board
                .get_square(&Position::new(file, Rank::Two))
                .expect("white pawn must exist");

            assert_eq!(pawn_square.piece.piece_type, PieceType::Pawn);
            assert_eq!(pawn_square.piece.piece_color, PieceColor::White);
        }

        for (i, expected_piece) in back_line.iter().enumerate() {
            let file = File::try_from(i).unwrap();

            let square = board
                .get_square(&Position::new(file, Rank::Eight))
                .expect("black back line piece must exist");

            assert_eq!(square.piece.piece_type, *expected_piece);
            assert_eq!(square.piece.piece_color, PieceColor::Black);

            let pawn_square = board
                .get_square(&Position::new(file, Rank::Seven))
                .expect("black pawn must exist");

            assert_eq!(pawn_square.piece.piece_type, PieceType::Pawn);
            assert_eq!(pawn_square.piece.piece_color, PieceColor::Black);
        }
    }
}
