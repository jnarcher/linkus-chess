// main.rs

use move_tables::gen_tables;
use negamax::negamax;
use parse::uci_loop;
use board::Board;
use piece_move::Move;

pub mod board;
pub mod negamax;
pub mod color;
pub mod piece_move;
pub mod piece;
pub mod bitboard;
pub mod evaluate;
pub mod move_tables;
pub mod sliding_attacks;
pub mod square;
pub mod parse;
pub mod perft;

fn main() {
    gen_tables();
    // uci_loop();

    let mut board = Board::start();
    // let mut board = Board::new(
    //     "rnbqkbnr/ppp1pppp/8/8/8/8/1p6/4K b - - 0 1"
    // ).unwrap();
    // board.make_move(Move::new(crate::square::Square::B2, crate::square::Square::B1, crate::piece_move::SpecialMove::Promotion(crate::piece::Piece::Queen)));

    println!("{board}");
    for _ in 0..10000 {
        board.gen_moves();
        let mv = negamax(&mut board, 4, false);
        board.make_move(mv);
        println!("{board}");
    }


}


