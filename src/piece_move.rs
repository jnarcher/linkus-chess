// piece_move.rs

use crate::piece::Piece;
use crate::square::Square;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SpecialMove {
    Quiet,
    DoublePush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    Promotion(Piece),
    PromotionCapture(Piece),
}

/// Uses 16-bit move structre from
/// https://www.chessprogramming.org/Encoding_Moves
/// 
///        111111   111111  origin -> target
/// 
///    0	0	0	0	0	quiet moves
///    1	0	0	0	1	double pawn push
///    2	0	0	1	0	king castle
///    3	0	0	1	1	queen castle
///    4	0	1	0	0	captures
///    5	0	1	0	1	ep-capture
///    8	1	0	0	0	knight-promotion
///    9	1	0	0	1	bishop-promotion
///    10	1	0	1	0	rook-promotion
///    11	1	0	1	1	queen-promotion
///    12	1	1	0	0	knight-promo capture
///    13	1	1	0	1	bishop-promo capture
///    14	1	1	1	0	rook-promo capture
///    15	1	1	1	1	queen-promo capture

#[derive(Copy, Clone)] 
pub struct Move(u16);

impl Move {
    pub fn new(
        origin: Square, 
        target: Square, 
        special_type: SpecialMove,
    ) -> Move {
        let mut mv = 0u16;
        mv |= origin.to_int() as u16;
        mv |= (target.to_int() as u16) << 6;

        let special: u16;
        match special_type {
            SpecialMove::Quiet            => special = 0,
            SpecialMove::DoublePush       => special = 1,
            SpecialMove::KingsideCastle   => special = 2,
            SpecialMove::QueensideCastle  => special = 3,
            SpecialMove::Capture          => special = 4,
            SpecialMove::EnPassant        => special = 5,
            SpecialMove::Promotion(piece) => {
                match piece {
                    Piece::Knight => special = 8,
                    Piece::Bishop => special = 9,
                    Piece::Rook   => special = 10,
                    Piece::Queen  => special = 11,
                    _ => panic!("Invalid promotion type.")
                }
            }
            SpecialMove::PromotionCapture(piece) => {
                match piece {
                    Piece::Knight => special = 12,
                    Piece::Bishop => special = 13,
                    Piece::Rook   => special = 14,
                    Piece::Queen  => special = 15,
                    _ => panic!("Invalid promotion type.")
                }
            }
        }
        
        Move(mv | (special << 12))
    }

    /// Get move origin square.
    #[inline] 
    pub fn get_origin(&self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }

    /// Get move target square.
    #[inline] 
    pub fn get_target(&self) -> Square {
        Square::new(((self.0 & (0b111111 << 6)) >> 6) as u8)
    } 


    /// Get special move type.
    pub fn get_special(&self) -> SpecialMove {
        match (self.0 & (0b1111 << 12)) >> 12 {
            0  => SpecialMove::Quiet,
            1  => SpecialMove::DoublePush,
            2  => SpecialMove::KingsideCastle,
            3  => SpecialMove::QueensideCastle,
            4  => SpecialMove::Capture,
            5  => SpecialMove::EnPassant,
            8  => SpecialMove::Promotion(Piece::Knight),
            9  => SpecialMove::Promotion(Piece::Bishop),
            10 => SpecialMove::Promotion(Piece::Rook),
            11 => SpecialMove::Promotion(Piece::Queen),
            12 => SpecialMove::PromotionCapture(Piece::Knight),
            13 => SpecialMove::PromotionCapture(Piece::Bishop),
            14 => SpecialMove::PromotionCapture(Piece::Rook),
            15 => SpecialMove::PromotionCapture(Piece::Queen),
            _  => panic!()
        }
    }

}


impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}{}",
            self.get_origin(),
            self.get_target(),
        }
    }
}


//~~~~~~~~~~~~~~~~~~ MOVE LIST ~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Clone)]
pub struct MoveList(Vec<Move>);

impl MoveList {

    /// Create empty MoveList.
    pub fn empty() -> MoveList {
        MoveList(vec![])
    }

    /// Push new move to top of stack.
    pub fn add(&mut self, mv: Move) {
        self.0.push(mv);
    } 

    /// Get last move and remove it from move list. 
    pub fn take(&mut self) -> Option<Move> {
        self.0.pop()
    } 

    /// Get last move without changing list.
    pub fn peek(&self) -> Option<Move> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0[self.0.len()])
        }
    }

    /// Get actual Vec to iterate over
    #[inline] 
    pub fn into_vec(&self) -> Vec<Move> {
        self.0.clone()
    }
}


impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for mv in &self.0 {
            display = format!("{}\n{}", display, mv)
        }

        write! { f, "{}", display}
    }
}
