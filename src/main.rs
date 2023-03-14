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
    uci_loop();
}


