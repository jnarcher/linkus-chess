use move_tables::gen_tables;
use board::Board;
use negamax::negamax;

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

    // let mut board = Board::start();
    let mut board = Board::new(
        "rnbqkbnr/ppp2ppp/4p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 1"
    ).unwrap();

    println!("{board}");


    for _ in 0..1000 {
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s);

        let mv = negamax(&mut board, 2, false);
        board.make_move(mv);
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input);

        // let mv = parse_input(&board, input);
        // if mv == NO_MOVE {continue};
        // println!("{mv}");

        // let mut found_move = false;
        // for m in board.get_move_list().into_vec() {
        //     if mv == m {
        //         found_move = true;
        //         break;
        //     }
        // }
        // if !found_move {
        //     println!("not found in move list Invalid move.");
        //     continue;
        // }

        // if !board.make_move(mv) {
        //     board = copy.clone();
        //     println!("Invalid move.")
        // }
        print!("\x1B[2J");
        println!("{board}");
    }
}


