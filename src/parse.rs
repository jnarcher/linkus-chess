// parse.rs

use crate::board::Board;
use crate::square::Square;
use crate::piece_move::*;
use crate::piece::Piece;

pub fn parse_input(board: &Board, input: String) -> Move {
    let mut s = input;
    s.pop();
    let origin = Square::from_alg(&s[0..2]).unwrap();
    let target = Square::from_alg(&s[2..4]).unwrap();

    let mut special = SpecialMove::Quiet;

    let piece_index = match board.get_bitboard_index_at_square(origin) {
        Some(inner) => {
            special = SpecialMove::Capture;
            inner
        },
        None => return NO_MOVE
    };

    if piece_index == 5 || piece_index == 11 {
        if s.len() == 5 {
            if special == SpecialMove::Capture {
                match &s[4..] {
                    "q" => {
                        special = SpecialMove::PromotionCapture(Piece::Queen);
                    },
                    "r" => {
                        special = SpecialMove::PromotionCapture(Piece::Rook);
                    },
                    "b" => {
                        special = SpecialMove::PromotionCapture(Piece::Bishop);
                    },
                    "n" => {
                        special = SpecialMove::PromotionCapture(Piece::Knight);
                    }
                    _ => return NO_MOVE,
                }
            } else {
                match &s[4..] {
                    "q" => {
                        special = SpecialMove::PromotionCapture(Piece::Queen);
                    },
                    "r" => {
                        special = SpecialMove::PromotionCapture(Piece::Rook);
                    },
                    "b" => {
                        special = SpecialMove::PromotionCapture(Piece::Bishop);
                    },
                    "n" => {
                        special = SpecialMove::PromotionCapture(Piece::Knight);
                    }
                    _ => return NO_MOVE,
                }
            }
        } else if (origin.to_int() as f32 - target.to_int() as f32).abs() == 16.0 {
            special = SpecialMove::DoublePush;
        } else {
            special = SpecialMove::Quiet;
        }
    } else if piece_index == 0 || piece_index == 6 {
        if target.to_int() as f32 - origin.to_int() as f32 == 2.0 {
            special = SpecialMove::KingsideCastle;
        } else if target.to_int() as f32 - origin.to_int() as f32 == -2.0 {
            special = SpecialMove::QueensideCastle;
        }
    }

    Move::new(origin, target, special)
}