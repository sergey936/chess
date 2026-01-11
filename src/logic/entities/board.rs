use crate::logic::entities::piece::Piece;
use crate::logic::entities::position::Position;

#[derive(Debug)]
pub enum MoveError {
    NoPieceAtSource,
    TargetSquareAlreadyHaveAlliedPiece,
}


pub struct Square {
    pub piece: Piece,
    pub position: Position,
}


pub struct Board {
    pub squares: [[Option<Square>; 8]; 8],
}

impl Board {
    pub fn create_empty_board() -> Board {
        Board {
            squares: std::array::from_fn(|_| {
                std::array::from_fn(|_| None)
            }),
        }
    }

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        let file = position.file.index();
        let rank = position.rank.index();

        self.squares[rank][file] = Some(Square { piece, position });
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        let from_file = from.file.index();
        let from_rank = from.rank.index();

        let to_file = to.file.index();
        let to_rank = to.rank.index();

        let square_from = self.squares[from_rank][from_file]
            .as_ref()
            .ok_or(MoveError::NoPieceAtSource)?;

        if let Some(square_to) = self.squares[to_rank][to_file].as_ref() {
            if square_to.piece.piece_color == square_from.piece.piece_color {
                return Err(MoveError::TargetSquareAlreadyHaveAlliedPiece);
            }
        };

        let square = self.squares[from_rank][from_file].take().unwrap();

        self.squares[to_rank][to_file] = Some(Square {
            piece: square.piece,
            position: to,
        });

        Ok(())
    }

    pub fn get_square(&self, position: &Position) -> Option<&Square> {
        let file = position.file.index();
        let rank = position.rank.index();

        let square = self.squares[rank][file].as_ref();

        square
    }
}
