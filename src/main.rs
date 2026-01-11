mod logic;

use crate::logic::board::builders::base_builder::BoardBuilder;
use crate::logic::board::builders::classic_builder::ClassicChessBoardBuilder;
use crate::logic::entities::position::{File, Position, Rank};
use crate::logic::move_rules::legal_moves::get_legal_moves;

fn main() {
    let mut game_board = ClassicChessBoardBuilder.build();
    game_board.move_piece(Position::new(File::E, Rank::Two), Position::new(File::E, Rank::Four)).expect("panic message");

    for line in game_board.squares.iter() {
        for square in line.iter() {
            match square {
                Some(square) => {
                    println!("Есть фигура {:?} на {:?} {:?}", square.piece.piece_type, square.position.file, square.position.rank);
                }
                None => {}
            };
        };
    };

    let legal_moves = get_legal_moves(&game_board, Position::new(File::E, Rank::Four));

    for legal_move in legal_moves.iter() {
        println!("{:?}", legal_move);
    };
}