// evaluate.rs

use crate::board::Board;
use crate::color::Color;

const PIECE_VALUES: [i32; 12] = [
     10000,
      1000,
       500,
       350,
       300,
       100,
    -10000,
     -1000,
      -500,
      -350,
      -300,
      -100,
];

const PST_PAWN: [i32; 64] = [
    90,  90,  90,  90,  90,  90,  90,  90,
    30,  30,  30,  40,  40,  30,  30,  30,
    20,  20,  20,  30,  30,  30,  20,  20,
    10,  10,  10,  20,  20,  10,  10,  10,
     0,   0,  10,  20,  20,   5,   5,   5,
     0,   0,   0,   5,   5,   0,   0,   0,
     0,   0,   0, -10, -10,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
];

const PST_KNIGHT: [i32; 64] = [
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,  10,  10,   0,   0,  -5,
    -5,   5,  20,  20,  20,  20,   5,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,   5,  20,  10,  10,  20,   5,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5, -10,   0,   0,   0,   0, -10,  -5,
];

const PST_BISHOP: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,  10,  10,   0,   0,   0,
    0,   0,  10,  20,  20,  10,   0,   0,
    0,   0,  10,  20,  20,  10,   0,   0,
    0,  10,   0,   0,   0,   0,  10,   0,
    0,  30,   0,   0,   0,   0,  30,   0,
    0,   0, -10,   0,   0, -10,   0,   0,
];

const PST_ROOK: [i32; 64] = [
    50,  50,  50,  50,  50,  50,  50,  50,
    50,  50,  50,  50,  50,  50,  50,  50,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,   0,  20,  20,   0,   0,   0,
];

const PST_KING: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   5,   5,   5,   5,   0,   0,
    0,   5,   5,  10,  10,   5,   5,   0,
    0,   5,  10,  20,  20,  10,   5,   0,
    0,   5,  10,  20,  20,  10,   5,   0,
    0,   0,   5,  10,  10,   5,   0,   0,
    0,   5,   5,  -5,  -5,   0,   5,   0,
    0,   0,   5,   0, -15,   0,  10,   0,
];

const MIRRORED: [usize; 64] = [
    63, 62, 61, 60, 59, 58, 57, 56, 
    55, 54, 53, 52, 51, 50, 49, 48, 
    47, 46, 45, 44, 43, 42, 41, 40, 
    39, 38, 37, 36, 35, 34, 33, 32, 
    31, 30, 29, 28, 27, 26, 25, 24, 
    23, 22, 21, 20, 19, 18, 17, 16, 
    15, 14, 13, 12, 11, 10,  9,  8, 
     7,  6,  5,  4,  3,  2,  1,  0, 
];


pub fn eval(board: &Board, color: Color) -> i32 {
    let mut score = 0;

    for i in 0..12 {
        let mut bb = board.bitboards[i].clone();
        score += PIECE_VALUES[i] * bb.count_bits() as i32; 

        loop {
            let square = match bb.get_lsb_square() {
                Some(sq) => sq,
                None => break,
            };

            match i {
                0 => score += PST_KING[square.to_index()],
                2 => score += PST_ROOK[square.to_index()],
                3 => score += PST_BISHOP[square.to_index()],
                4 => score += PST_KNIGHT[square.to_index()],
                5 => score += PST_PAWN[square.to_index()],
                6 => score -= PST_KING[MIRRORED[square.to_index()]],
                8 => score -= PST_ROOK[MIRRORED[square.to_index()]],
                9 => score -= PST_BISHOP[MIRRORED[square.to_index()]],
                10 => score -= PST_KNIGHT[MIRRORED[square.to_index()]],
                11 => score -= PST_PAWN[MIRRORED[square.to_index()]],
                _ => {}
            }

            bb.pop_bit_by_square(square);
        }

    }
    match color {
        Color::WHITE => { return score },
        Color::BLACK => { return -score },
    }
}