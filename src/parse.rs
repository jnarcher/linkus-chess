// parse.rs

use crate::board::Board;
use crate::negamax::negamax;
use crate::perft::perft;
use crate::square::Square;
use crate::piece_move::*;
use crate::piece::Piece;

pub fn uci_loop() {
    let mut board = Board::start();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input.");

        let chars: Vec<char> = input
            .clone()
            .chars()
            .collect();

        // check for "d" -> display board
        let command: String = chars.iter().take(1).collect();
        if &command == "d" {
            println!("{board}");
            continue
        }

        // check for "isready"
        let command: String = chars.iter().take(7).collect();
        if &command == "isready" {
            println!("readyok");
            continue
        }

        // check for "position"
        let command: String = chars.iter().take(8).collect();
        if &command == "position" {
            board = match parse_position(&input) {
                Some(b) => b,
                None => { board },
            };
            continue
        }

        // check for "ucinewgame"
        let command: String = chars.iter().take(10).collect();
        if &command == "ucinewgame" {
            board = Board::start();
            continue
        }


        // check for "go"
        let command: String = chars.iter().take(2).collect();
        if &command == "go" {
            parse_go(&mut board, &input);
            continue
        }

        // check for "quit"
        let command: String = chars.iter().take(4).collect();
        if &command == "quit" {
            break
        }

        // check for "uci"
        let command: String = chars.iter().take(3).collect();
        if &command == "uci" {
            println!("id name linkus");
            println!("id author Jalen Archer");
            println!("uciok");
            continue
        }

        println!("Unknown command.");
    }
}

fn parse_move(board: &mut Board, input: &str) -> Option<Move> {
    board.gen_moves();

    let mut chars = input.chars();

    let mut first_square = chars
        .next()
        .unwrap()
        .to_string();

    first_square.push(
        chars.next().unwrap()
    );
    let origin = Square::from_alg(&first_square).unwrap();

    let mut second_square = chars
        .next()
        .unwrap()
        .to_string();

    second_square.push(
        chars.next().unwrap()
    );
    let target = Square::from_alg(&second_square).unwrap();

    let promoted_type = match chars.next() {
        Some(c) => {
            if c != '\n' {
                c
            } else {
                ' '
            }
        },
        None => {
            ' '
        }
    };

    for mv in board.get_move_list().into_vec() {
        if mv.get_origin() != origin || mv.get_target() != target {
            continue
        }

        match mv.get_special() {
            SpecialMove::Promotion(p) => {
                match p {
                    Piece::Queen => {
                        if promoted_type == 'q' {
                            return Some(mv)
                        }
                    },
                    Piece::Rook => {
                        if promoted_type == 'r' {
                            return Some(mv)
                        }
                    },
                    Piece::Bishop => {
                        if promoted_type == 'b' {
                            return Some(mv)
                        }
                    },
                    Piece::Knight => {
                        if promoted_type == 'n' {
                            return Some(mv)
                        }
                    },
                    _ => continue
                }
            },
            SpecialMove::PromotionCapture(p) => {
                match p {
                    Piece::Queen => {
                        if promoted_type == 'q' {
                            return Some(mv)
                        }
                    },
                    Piece::Rook => {
                        if promoted_type == 'r' {
                            return Some(mv)
                        }
                    },
                    Piece::Bishop => {
                        if promoted_type == 'b' {
                            return Some(mv)
                        }
                    },
                    Piece::Knight => {
                        if promoted_type == 'n' {
                            return Some(mv)
                        }
                    },
                    _ => continue
                }
            },
            _ => {
                if promoted_type == ' ' { 
                    return Some(mv) 
                } else { 
                    return None 
                }
            }
        }
    }
    None
}

fn parse_position(input: &str) -> Option<Board> {

    
    let mut chars: Vec<char> = input
        .chars()
        .collect();


    // skip "position"
    chars = chars[9..].to_vec();

    let mut board = Board::start();

    // check if fen
    let command: String = chars.iter().take(3).collect();
    if &command == "fen" {

        // skip "fen"
        chars = chars[4..].to_vec();

        // if fen string given update board position
        if chars.len() != 0 {
            board = match Board::new(
                &chars.iter().collect::<String>()
            ) {
                Ok(b) => b,
                Err(_) => return None,
            };
        }

        // skip fen string
        let mut space_count = 0;
        let mut skip_num = 0;
        for (i, c) in chars.iter().enumerate() {
            if space_count == 6 {
                skip_num = i;
                break;
            }
            if *c == ' ' {
                space_count += 1;
            } 
        } 
        chars = chars[skip_num..].to_vec();

    } else {

        // check if startpos
        let command: String = chars.iter().take(8).collect();

        if &command != "startpos" {
            return None
        }

        // skip "startpos"
        chars = chars[9..].to_vec();
    }

    // check if moves are provided
    let new_s = chars.iter().collect::<String>();
    match new_s.split_whitespace().find(|&x| x == "moves") {
        Some(_) => {

            // skip "moves"
            chars = chars[6..].to_vec();

            // get list of move strings
            let moves_str: String = chars.iter().collect::<String>();
            let move_list: Vec<&str> = moves_str.split_whitespace().collect::<Vec<_>>();

            for m in move_list {
                let mv = parse_move(&mut board, m).unwrap();
                board.make_move(mv);
            }

        },
        None => {}
    }

    Some(board)
}

fn parse_go(board: &mut Board, input: &str) {

    let mut chars: Vec<char> = input
        .chars()
        .collect();

    // skip "go"
    chars = chars[3..].to_vec();

    // "depth" command
    let command: String = chars.iter().take(5).collect();
    if &command == "depth" {
        chars = chars[6..].to_vec();

        let depth = match chars.iter().collect::<String>().trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let mv = negamax(board, depth as u8, false);
        println!("bestmove {mv}");
        // println!("bestmove d2d4");
        return;
    }

    // "perft" command
    let command: String = chars.iter().take(5).collect();
    if &command == "perft" {
        chars = chars[6..].to_vec();

        let depth = match chars.iter().collect::<String>().trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => return,
        };

        perft(board, depth as u8);
        return;
    }

}
