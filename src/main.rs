mod logic;

use crate::logic::board::builders::base_builder::BoardBuilder;
use crate::logic::board::builders::classic_builder::ClassicChessBoardBuilder;


fn main() {
    let game_board = ClassicChessBoardBuilder.new();
    for line in game_board.squares.iter() {
        for square in line.iter() {
            match square {
                Some(square) => {
                    println!("Есть фигура {:?} на {:?} {:?}", square.piece.piece_type, square.position.file, square.position.rank);
                }
                None => {}
            }
        }
    }
}