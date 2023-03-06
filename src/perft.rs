// perft.rs

use crate::board::Board;

pub fn perft_driver(board: &mut Board, depth: u8) -> u128 {
    let mut leafs = 0;
    if depth == 0 {
        return 1;
    }
    let copy = board.clone();
    board.gen_moves();
    for mv in board.get_move_list().into_vec() {
        if board.make_move(mv) {
            leafs += perft_driver(board, depth - 1);
        }
        *board = copy.clone();
    }
    leafs
}

pub fn perft(board: &mut Board, depth: u8) {
    let mut total: u128 = 0;
    board.gen_moves();
    let copy = board.clone();
    for mv in board.get_move_list().into_vec() {
        let mut nodes = 0;
        if board.make_move(mv) {
            nodes += perft_driver(board, depth - 1);
            total += nodes;
            print!("{mv}");
            print!(": {}\n", nodes);
        }
        *board = copy.clone();
    }

    println!("\nNodes:  {}", total);
}
