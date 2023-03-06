// negamax.rs

use crate::board::Board;
use crate::color::Color;
use crate::evaluate::eval;
use crate::piece_move::*;

pub fn negamax_driver(board: &mut Board, depth: u8) -> i32 {

    if depth == 0 { return eval(board, board.get_to_move()) }
    let mut best_score = i32::MIN;
    let mut legal_moves = 0u8;
    board.gen_moves();

    let copy = board.clone();
    for mv in board.get_move_list().into_vec() {

        if board.make_move(mv) {
            let score = -negamax_driver(board, depth - 1);
            if score > best_score {
                best_score = score;
            }
            legal_moves += 1;
        }
        // take back move
        *board = copy.clone();
    }

    // check for mate
    if legal_moves == 0 {
        if board.is_in_check() {
            match board.get_to_move() {
                Color::WHITE => {
                    return 10000000
                },
                Color::BLACK => {
                    return -10000000
                }
            }
        } else {
            return 0;
        }
    }

    return best_score;

}

pub fn negamax(board: &mut Board, depth: u8, print: bool) -> Move {

    let mut best_move = NO_MOVE;
    let mut best_score = i32::MIN;
    board.gen_moves();

    let copy = board.clone();
    for mv in board.get_move_list().into_vec() {
        if board.make_move(mv) {
            let score = -negamax_driver(board, depth-1);
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
            if print {
                println!("{mv}: {score}");
            }
        }
        *board = copy.clone();
    }

    if print {
        println!("\nBest move: {best_move}\nScore: {best_score}");
    }

    return best_move;
}