// main.rs

// use crate::bitboard::*;
// use std::time::{Instant, Duration};
// use crate::square::*;
use crate::move_tables::gen_tables;
// use crate::color::*;
use crate::board::*;
use crate::perft::*;
// use crate::piece::*;
// use crate::piece_move::*;
// use std::io;
pub mod piece_move;
pub mod bitboard;
pub mod board;
pub mod square;
pub mod move_tables;
pub mod color;
pub mod sliding_attacks;
pub mod piece;
pub mod perft;

fn main() {
    // initialize lookup tables
    gen_tables();

    let mut board = Board::start();
    println!("{board}");

    perft(&mut board, 2);

}
