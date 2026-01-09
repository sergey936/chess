use crate::logic::board::builders::base_builder::BoardBuilder;
use crate::logic::entities::board::Board;
use crate::logic::entities::piece::{Piece, PieceColor, PieceType};
use crate::logic::entities::position::{File, Position, Rank};

pub struct ClassicChessBoardBuilder;

impl BoardBuilder for ClassicChessBoardBuilder {
    fn new(&self) -> Board {
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
                Piece::new(PieceType::Pawn, PieceColor::Black),
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