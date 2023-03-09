// board.rs

use std::fmt;
use crate::move_tables::KING_ATTACKS;
use crate::move_tables::KNIGHT_ATTACKS;
use crate::move_tables::PAWN_ATTACKS;
use crate::piece::*;
use crate::color::*;
use crate::bitboard::*;
use crate::piece_move::MoveList;
use crate::piece_move::SpecialMove;
use crate::sliding_attacks::get_bishop_rays;
use crate::sliding_attacks::get_queen_rays;
use crate::sliding_attacks::get_rook_rays;
use crate::square::ALL_SQUARES;
use crate::square::Square;
use crate::piece_move::Move;


pub const STARTPOS: &str = 
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


#[derive(Debug)]
pub enum FenParseError {
    InvalidPosition,
    SideToMove,
    Castling,
    EnPassant,
    HalfMove,
    Ply,
}

/// Tracks current game state
#[derive(Clone)] 
pub struct Board {
    pub bitboards: [Bitboard; 12],
    to_move: Color,
    castling_rights: u8,
    en_passant: Square,
    in_check: bool,
    pub pseudo_moves: MoveList,
}

// Don't want the move generation to be dependent on the fen string

impl Board {

    /// Creates a new board state from FEN string.
    pub fn new(fen: &str) -> Result<Board, FenParseError> {

        // define empty bitboard set
        let mut bitboards: [Bitboard; 12] = [EMPTY; 12];

        // fill bitboards
        let position = fen.split_whitespace().nth(0).unwrap();
        let mut index = 0u64;
        for c in position.chars() {
            match c {
                'K' => bitboards[0]  |= 1u64 << index,
                'Q' => bitboards[1]  |= 1u64 << index,
                'R' => bitboards[2]  |= 1u64 << index,
                'B' => bitboards[3]  |= 1u64 << index,
                'N' => bitboards[4]  |= 1u64 << index,
                'P' => bitboards[5]  |= 1u64 << index,
                'k' => bitboards[6]  |= 1u64 << index,
                'q' => bitboards[7]  |= 1u64 << index,
                'r' => bitboards[8]  |= 1u64 << index,
                'b' => bitboards[9]  |= 1u64 << index,
                'n' => bitboards[10] |= 1u64 << index,
                'p' => bitboards[11] |= 1u64 << index,
                x if x.is_numeric() => {
                    index += x.to_string().parse::<u64>().unwrap();
                    continue;
                }
                '/' => continue,
                _ => return Err(FenParseError::InvalidPosition),
            }
            if c.is_alphabetic() {index += 1}
        }

        // make sure two kings are on the board
        if bitboards[0].count_bits() != 1 || 
           bitboards[6].count_bits() != 1 {
            return Err(FenParseError::InvalidPosition);
        }

        let to_move = match fen.split_whitespace().nth(1).unwrap() {
            "w" => Color::WHITE,
            "b" => Color::BLACK,
             _ => { return Err(FenParseError::SideToMove); }
        };

        let mut flag = false;
        let castling = fen.split_whitespace().nth(2).unwrap();
        let mut castling_rights: u8 = 0;

        // Check if there are characters except KQkd- in castling field
        for c in castling.chars() {
            match c {
                'K' => continue,
                'Q' => continue,
                'k' => continue,
                'q' => continue,
                '-' => continue,
                 _  => flag = true,
            }
        }

        if castling != "-" {
            match castling.find("K") {
                Some(_) => {
                    castling_rights += 8;
                } 
                None => {},
            }
            match castling.find("Q") {
                Some(_) => {
                    castling_rights += 4;
                } 
                None => {},
            }
            match castling.find("k") {
                Some(_) => {
                    castling_rights += 2;
                } 
                None => {},
            }
            match castling.find("q") {
                Some(_) => {
                    castling_rights += 1;
                } 
                None => {},
            }
        }
        if flag { return Err(FenParseError::Castling) } 



        let en_pass_square = fen.split_whitespace().nth(3).unwrap();
        let en_passant: Square;
        if en_pass_square == "-" {
            en_passant = Square::NO_SQUARE;
        } else {
            en_passant = match Square::from_alg(en_pass_square) {
                Some(square) => square,
                None => return Err(FenParseError::EnPassant)
            }
        }

        match fen
            .split_whitespace()
            .nth(4)
            .unwrap()
            .parse::<u8>() {

            Ok(inner) => inner,
            Err(_) => {
                return Err(FenParseError::HalfMove);
            }

        };

        match fen.split_whitespace().nth(5).unwrap().parse::<u64>() {
            Ok(inner) => {
                if inner == 0 {
                    return Err(FenParseError::Ply);
                }
                match to_move {
                    Color::WHITE => inner*2-1,
                    Color::BLACK => inner*2
                }
            }
            Err(_) => {
                return Err(FenParseError::Ply);
            }
        };

        // set fields
        Ok(Board {
            bitboards,
            to_move,
            castling_rights,
            en_passant,
            in_check: false,
            pseudo_moves: MoveList::empty(),
        })
    }

    /// Creates a new board from the starting position.
    pub fn start() -> Board {
        Board::new(STARTPOS).unwrap()
    }

    /// Get move list.
    pub fn get_move_list(&self) -> MoveList {
        self.pseudo_moves.clone()
    }

    /// Get color to move.
    pub fn get_to_move(&self) -> Color {
        self.to_move
    }

    /// Get whether the king of the current to move color is in check.
    pub fn is_in_check(&self) -> bool {
        self.in_check
    }

    /// Get bitboard containing all pieces.
    pub fn get_all_pieces(&self) -> Bitboard {
        let mut all = EMPTY;
        for bb in self.bitboards {
            all |= bb;
        }
        all
    }

    /// Get bitboard containing all white pieces.
    pub fn get_white_pieces(&self) -> Bitboard {
        let mut white = EMPTY;
        for bb in &self.bitboards[..6] {
            white |= bb;
        }
        white
    }

    /// Get bitboard containing all black pieces.
    pub fn get_black_pieces(&self) -> Bitboard {
        let mut black = EMPTY;
        for bb in &self.bitboards[6..] {
            black |= bb;
        }
        black
    }

    /// Check if square is attacked by color.
    pub fn is_attacked(&self, square: Square, color: Option<Color>) -> bool {
        // return if `square` is attacked by `color`

        match color {
            Some(Color::WHITE) => {
                // pawns
                unsafe {
                    if !(PAWN_ATTACKS[1][square.to_index()] & 
                    self.bitboards[5]).is_empty() {
                        return true;     
                    }

                    // knights
                    if !(KNIGHT_ATTACKS[square.to_index()] &
                    self.bitboards[4]).is_empty() {
                        return true;
                    }

                    // king
                    if !(KING_ATTACKS[square.to_index()] &
                    self.bitboards[0]).is_empty() {
                        return true;
                    }
                }

                let all_pieces = self.get_all_pieces();
                // bishops
                if !(get_bishop_rays(square, all_pieces) &
                self.bitboards[3]).is_empty() {
                    return true;
                }

                // rooks
                if !(get_rook_rays(square, all_pieces) &
                self.bitboards[2]).is_empty() {
                    return true;
                }

                // queens
                if !(get_queen_rays(square, all_pieces) &
                self.bitboards[1]).is_empty() {
                    return true;
                }
            },
            Some(Color::BLACK) => {
                unsafe {
                    // pawns
                    if !(PAWN_ATTACKS[0][square.to_index()] & 
                    self.bitboards[11]).is_empty() {
                        return true;     
                    }

                    // knights
                    if !(KNIGHT_ATTACKS[square.to_index()] &
                    self.bitboards[10]).is_empty() {
                        return true;
                    }

                    // king
                    if !(KING_ATTACKS[square.to_index()] &
                    self.bitboards[6]).is_empty() {
                        return true;
                    }
                }

                let all_pieces = self.get_all_pieces();
                // bishops
                if !(get_bishop_rays(square, all_pieces) &
                self.bitboards[9]).is_empty() {
                    return true;
                }

                // rooks
                if !(get_rook_rays(square, all_pieces) &
                self.bitboards[8]).is_empty() {
                    return true;
                }

                // queens
                if !(get_queen_rays(square, all_pieces) &
                self.bitboards[7]).is_empty() {
                    return true;
                }
            },
            _ => {
                unsafe {
                    // pawns
                    if !(PAWN_ATTACKS[1][square.to_index()] & 
                    self.bitboards[5]).is_empty() {
                        return true;     
                    }
                    if !(PAWN_ATTACKS[0][square.to_index()] & 
                    self.bitboards[11]).is_empty() {
                        return true;     
                    }

                    // knights
                    if !(KNIGHT_ATTACKS[square.to_index()] &
                    self.bitboards[4]).is_empty() {
                        return true;
                    }
                    if !(KNIGHT_ATTACKS[square.to_index()] &
                    self.bitboards[10]).is_empty() {
                        return true;
                    }

                    // king
                    if !(KING_ATTACKS[square.to_index()] &
                    self.bitboards[0]).is_empty() {
                        return true;
                    }
                    if !(KING_ATTACKS[square.to_index()] &
                    self.bitboards[6]).is_empty() {
                        return true;
                    }
                }

                let all_pieces = self.get_all_pieces();
                // bishops
                if !(get_bishop_rays(square, all_pieces) &
                self.bitboards[3]).is_empty() {
                    return true;
                }
                if !(get_bishop_rays(square, all_pieces) &
                self.bitboards[9]).is_empty() {
                    return true;
                }

                // rooks
                if !(get_rook_rays(square, all_pieces) &
                self.bitboards[2]).is_empty() {
                    return true;
                }
                if !(get_rook_rays(square, all_pieces) &
                self.bitboards[8]).is_empty() {
                    return true;
                }

                // queens
                if !(get_queen_rays(square, all_pieces) &
                self.bitboards[1]).is_empty() {
                    return true;
                }
                if !(get_queen_rays(square, all_pieces) &
                self.bitboards[7]).is_empty() {
                    return true;
                }
            }
        }
        false
    }

    /// Generate bitboard of all squares attacked by color.
    pub fn get_attacked(&self, color: Option<Color>) -> Bitboard {
        let mut bb = EMPTY;
        for sq in ALL_SQUARES {
            if self.is_attacked(sq, color) {
                bb |= Bitboard::from_square(sq)
            }
        }
        bb
    }

    /// Get the bitboard index at square.
    pub fn get_bitboard_index_at_square(&self, square: Square) -> Option<usize> {
        for (i, bb) in self.bitboards.iter().enumerate() {
            if bb.get_bit_by_square(square) == 1 { 
                return Some(i)
            }
        }
        None
    } 

    /// Remove piece at square.
    pub fn del_piece(&mut self, square: Square) {
        let i = match self.get_bitboard_index_at_square(square) {
            None => return,
            Some(p) => p
        };
        self.bitboards[i].pop_bit_by_square(square);
    } 

    /// Generate pseudo legal moves.
    pub fn gen_moves(&mut self) {
        // TODO: ignore all moves that don't block/capture checking pieces or move king.
        // reset move_list
        self.pseudo_moves = MoveList::empty();

        let all_pieces = self.get_all_pieces();
        let white_pieces = self.get_white_pieces();
        let black_pieces = self.get_black_pieces();

        match self.to_move {
            Color::WHITE => {
                // pawns
                let mut bb = self.bitboards[5].clone(); // get white
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let target = Square::new((origin.to_int() - 8) as u8);
                    
                    if all_pieces.get_bit_by_square(target) == 0 {
                        if origin.rank() == 7 {
                            // quiet promotions
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Queen)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Rook)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Bishop)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Knight)));
                        } else {
                            // quiet push
                            self.pseudo_moves
                                .add(Move::new(origin, target,
                                     SpecialMove::Quiet));
                            if origin.rank() == 2 &&
                                all_pieces.get_bit(origin.to_int() as u8 - 16) != 1 {
                                    // double push
                                    self.pseudo_moves
                                        .add(Move::new(origin,
                                            Square::new(target.to_int() - 8),
                                            SpecialMove::DoublePush));
                            }
                        }
                    }

                    unsafe {
                        let mut attacks = PAWN_ATTACKS[0][origin.to_index()]
                            .clone();
                        // en passant
                        if self.en_passant != Square::NO_SQUARE && 
                        attacks.get_bit_by_square(self.en_passant) == 1 {
                            self.pseudo_moves
                                .add(Move::new(origin, self.en_passant,
                                     SpecialMove::EnPassant))  
                        }

                        attacks &= black_pieces;
                        loop {
                            let target = match attacks.get_lsb_square() {
                                Some(sq) => sq,
                                None => break
                            };
                            if target.rank() == 8 {
                                // capture promotion
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Queen)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Rook)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Bishop)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Knight)));
                            } else {
                                // capture
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::Capture))
                            }
                            attacks.pop_bit_by_square(target);
                        }
                    }
                    bb.pop_bit_by_square(origin);
                }

                // knights
                let mut bb = self.bitboards[4].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    unsafe {
                        let mut attacks = KNIGHT_ATTACKS[origin.to_index()]
                            .clone();

                        attacks ^= attacks & white_pieces;

                        loop {
                            let target = match attacks.get_lsb_square() {
                                Some(square) => square,
                                None => break
                            };
                            if black_pieces.get_bit_by_square(target) == 1 {
                                // capture
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin,
                                        target,
                                        SpecialMove::Capture))
                            } else {
                                // quiet
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin,
                                        target,
                                        SpecialMove::Quiet))
                            }
                            attacks.pop_bit_by_square(target);
                        }
                    }
                    bb.pop_bit_by_square(origin);
                }

                // king
                let bb = self.bitboards[0].clone();
                let origin = bb.get_lsb_square().unwrap();
                unsafe {
                    let mut attacks = KING_ATTACKS[origin.to_index()]
                        .clone();

                    attacks ^= attacks & white_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if black_pieces.get_bit_by_square(target) == 1 {
                            // capture
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Capture))
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                }

                // kingside castling
                if self.castling_rights & 8 != 0 {
                    let between = Bitboard::from_square(Square::F1) |
                                  Bitboard::from_square(Square::G1);
                    if (all_pieces & between).is_empty() {
                        if !self.is_attacked(Square::F1, Some(Color::BLACK)) &&
                        !self.in_check {
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    Square::new(origin.to_int() + 2),
                                    SpecialMove::KingsideCastle,
                                ))
                        }
                    }
                }

                // queen castling
                if self.castling_rights & 4 != 0 {
                    let between = Bitboard::from_square(Square::B1) |
                                  Bitboard::from_square(Square::C1) |
                                  Bitboard::from_square(Square::D1);
                    if (all_pieces & between).is_empty() {
                        if !self.is_attacked(Square::D1, Some(Color::BLACK)) &&
                        !self.in_check {
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    Square::new(origin.to_int() - 2),
                                    SpecialMove::QueensideCastle,
                                ))
                        }
                    }
                }

                // bishop
                let mut bb = self.bitboards[3].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_bishop_rays(origin, all_pieces);
                    
                    attacks ^= attacks & white_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if black_pieces.get_bit_by_square(target) == 1 {
                            // capture
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Capture))
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet
                                ))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }

                // rook
                let mut bb = self.bitboards[2].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_rook_rays(origin, all_pieces);

                    attacks ^= attacks & white_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if black_pieces.get_bit_by_square(target) == 1 {
                           // capture
                           self.pseudo_moves
                           .add(Move::new(
                               origin,
                               target,
                               SpecialMove::Capture)) 
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }

                // queen
                let mut bb = self.bitboards[1].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_queen_rays(origin, all_pieces);

                    attacks ^= attacks & white_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if black_pieces.get_bit_by_square(target) == 1 {
                           // capture
                           self.pseudo_moves
                           .add(Move::new(
                               origin,
                               target,
                               SpecialMove::Capture)) 
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }
            },
            Color::BLACK => {
                // pawns
                let mut bb = self.bitboards[11].clone(); // get white
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let target = Square::new((origin.to_int() + 8) as u8);
                    
                    if all_pieces.get_bit_by_square(target) == 0 {
                        if origin.rank() == 2 {
                            // quiet promotions
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Queen)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Rook)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Bishop)));
                            self.pseudo_moves
                                .add(Move::new(origin, target, 
                                     SpecialMove::Promotion(Piece::Knight)));
                        } else {
                            // quiet push
                            self.pseudo_moves
                                .add(Move::new(origin, target,
                                     SpecialMove::Quiet));
                            if origin.rank() == 7 &&
                            all_pieces.get_bit(origin.to_int() as u8 + 16) != 1 {
                                    // double push
                                    self.pseudo_moves
                                        .add(Move::new(origin,
                                            Square::new(target.to_int() + 8),
                                            SpecialMove::DoublePush));
                            }
                        }
                    }

                    unsafe {
                        let mut attacks = PAWN_ATTACKS[1][origin.to_index()]
                            .clone();
                        // en passant
                        if self.en_passant != Square::NO_SQUARE && 
                        attacks.get_bit_by_square(self.en_passant) == 1 {
                            self.pseudo_moves
                                .add(Move::new(origin, self.en_passant,
                                     SpecialMove::EnPassant))  
                        }

                        attacks &= white_pieces;
                        loop {
                            let target = match attacks.get_lsb_square() {
                                Some(sq) => sq,
                                None => break
                            };
                            if target.rank() == 1 {
                                // capture promotion
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Queen)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Rook)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Bishop)));
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::
                                        PromotionCapture(Piece::Knight)));
                            } else {
                                // capture
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin, 
                                        target, 
                                        SpecialMove::Capture))
                            }
                            attacks.pop_bit_by_square(target);
                        }
                    }
                    bb.pop_bit_by_square(origin);
                }

                // knights
                let mut bb = self.bitboards[10].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    unsafe {
                        let mut attacks = KNIGHT_ATTACKS[origin.to_index()]
                            .clone();

                        attacks ^= attacks & black_pieces;

                        loop {
                            let target = match attacks.get_lsb_square() {
                                Some(square) => square,
                                None => break
                            };
                            if white_pieces.get_bit_by_square(target) == 1 {
                                // capture
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin,
                                        target,
                                        SpecialMove::Capture))
                            } else {
                                // quiet
                                self.pseudo_moves
                                    .add(Move::new(
                                        origin,
                                        target,
                                        SpecialMove::Quiet))
                            }
                            attacks.pop_bit_by_square(target);
                        }
                    }
                    bb.pop_bit_by_square(origin);
                }

                // king
                let bb = self.bitboards[6].clone();
                let origin = bb.get_lsb_square().unwrap();
                unsafe {
                    let mut attacks = KING_ATTACKS[origin.to_index()]
                        .clone();

                    attacks ^= attacks & black_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if white_pieces.get_bit_by_square(target) == 1 {
                            // capture
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Capture))
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                }

                // kingside castling
                if self.castling_rights & 2 != 0 {
                    let between = Bitboard::from_square(Square::F8) |
                                  Bitboard::from_square(Square::G8);
                    if (all_pieces & between).is_empty() {
                        if !self.is_attacked(Square::F8, Some(Color::WHITE)) &&
                        !self.in_check {
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    Square::new(origin.to_int() + 2),
                                    SpecialMove::KingsideCastle,
                                ))
                        }
                    }
                }

                // queen castling
                if self.castling_rights & 1 != 0 {
                    let between = Bitboard::from_square(Square::B8) |
                                  Bitboard::from_square(Square::C8) |
                                  Bitboard::from_square(Square::D8);
                    if (all_pieces & between).is_empty() {
                        if !self.is_attacked(Square::D8, Some(Color::WHITE)) &&
                        !self.in_check {
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    Square::new(origin.to_int() - 2),
                                    SpecialMove::QueensideCastle,
                                ))
                        }
                    }
                }

                // bishop
                let mut bb = self.bitboards[9].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_bishop_rays(origin, all_pieces);
                    
                    attacks ^= attacks & black_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if white_pieces.get_bit_by_square(target) == 1 {
                            // capture
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Capture))
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet
                                ))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }

                // rook
                let mut bb = self.bitboards[8].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_rook_rays(origin, all_pieces);

                    attacks ^= attacks & black_pieces;

                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if white_pieces.get_bit_by_square(target) == 1 {
                           // capture
                           self.pseudo_moves
                           .add(Move::new(
                               origin,
                               target,
                               SpecialMove::Capture)) 
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }

                // queen
                let mut bb = self.bitboards[7].clone();
                loop {
                    let origin = match bb.get_lsb_square() {
                        Some(sq) => sq,
                        None => break
                    };
                    let mut attacks = get_queen_rays(origin, all_pieces);

                    attacks ^= attacks & black_pieces;
 
                    loop {
                        let target = match attacks.get_lsb_square() {
                            Some(sq) => sq,
                            None => break
                        };
                        if white_pieces.get_bit_by_square(target) == 1 {
                           // capture
                           self.pseudo_moves
                           .add(Move::new(
                               origin,
                               target,
                               SpecialMove::Capture)) 
                        } else {
                            // quiet
                            self.pseudo_moves
                                .add(Move::new(
                                    origin,
                                    target,
                                    SpecialMove::Quiet))
                        }
                        attacks.pop_bit_by_square(target);
                    }
                    bb.pop_bit_by_square(origin);
                }
            },
        }
    }

    /// Make a move and return if it is valid.
    pub fn make_move(&mut self, mv: Move) -> bool {

        let origin = mv.get_origin();
        let target = mv.get_target();

        // update castling rights if rooks are taken
        match self.get_bitboard_index_at_square(target) {
            Some(i) => {
                if i == 2 {
                    if target == Square::A1 {
                        self.castling_rights &= 0b1011;        
                    } else if target == Square::H1 {
                        self.castling_rights &= 0b0111;        
                    }
                } else if i == 8 {
                    if target == Square::A8 {
                        self.castling_rights &= 0b1110;        
                    } else if origin == Square::H8 {
                        self.castling_rights &= 0b1101;        
                    }
                }
            },
            None => {}
        };

        // handle special moves.
        let mut new_en_passant = false;
        match mv.get_special() {
            SpecialMove::Quiet => {
                self.en_passant = Square::NO_SQUARE;
            },
            SpecialMove::DoublePush => {
                match self.to_move {
                    Color::WHITE => {
                        self.en_passant = origin.due_n().unwrap();
                    },
                    Color::BLACK => {
                        self.en_passant = origin.due_s().unwrap();
                    }
                }
                new_en_passant = true;
            },
            SpecialMove::KingsideCastle => {
                // update castling rights
                match self.to_move {
                    Color::WHITE => {
                        self.castling_rights &= 0b0011;

                        // move rook
                        self.bitboards[2]
                            .move_bit_by_square(Square::H1, Square::F1);
                    },
                    Color::BLACK => {
                        self.castling_rights &= 0b1100;

                        // move rook
                        self.bitboards[8]
                            .move_bit_by_square(Square::H8, Square::F8);
                    },
                }
            },
            SpecialMove::QueensideCastle => {
                // update castling rights
                match self.to_move {
                    Color::WHITE => {
                        self.castling_rights &= 0b0011;

                        // move rook
                        self.bitboards[2]
                            .move_bit_by_square(Square::A1, Square::D1);
                    },
                    Color::BLACK => {
                        self.castling_rights &= 0b1100;

                        // move rook
                        self.bitboards[8]
                            .move_bit_by_square(Square::A8, Square::D8);
                    },
                }
            },
            SpecialMove::Capture => {
                self.del_piece(target);
            },
            SpecialMove::EnPassant => {
                match self.to_move {
                    Color::WHITE => {
                        self.bitboards[11]
                            .pop_bit_by_square(target.due_s().unwrap());
                    },
                    Color::BLACK => {
                        self.bitboards[5]
                            .pop_bit_by_square(target.due_n().unwrap());
                    }
                }
            },
            SpecialMove::Promotion(piece) => {
                match self.to_move {
                    Color::WHITE => {
                        // remove pawn
                        self.bitboards[5].pop_bit_by_square(origin);

                        // add new piece
                        match piece {
                            Piece::Queen => {
                                self.bitboards[1].set_bit_by_square(target);
                            },
                            Piece::Rook => {
                                self.bitboards[2].set_bit_by_square(target);
                            }
                            Piece::Bishop => {
                                self.bitboards[3].set_bit_by_square(target);
                            }
                            Piece::Knight => {
                                self.bitboards[4].set_bit_by_square(target);
                            }
                            _ => return false,
                        }
                    },
                    Color::BLACK => {
                        // remove pawn
                        self.bitboards[11].pop_bit_by_square(target);

                        // add new piece
                        match piece {
                            Piece::Queen => {
                                self.bitboards[7].set_bit_by_square(target);
                            },
                            Piece::Rook => {
                                self.bitboards[8].set_bit_by_square(target);
                            }
                            Piece::Bishop => {
                                self.bitboards[9].set_bit_by_square(target);
                            }
                            Piece::Knight => {
                                self.bitboards[10].set_bit_by_square(target);
                            }
                            _ => return false,
                        }
                    }
                }
            },
            SpecialMove::PromotionCapture(piece) => {
                self.del_piece(target);
                self.del_piece(origin);
                match self.to_move {
                    Color::WHITE => {
                        // add new piece
                        match piece {
                            Piece::Queen => {
                                self.bitboards[1].set_bit_by_square(target);
                            },
                            Piece::Rook => {
                                self.bitboards[2].set_bit_by_square(target);
                            }
                            Piece::Bishop => {
                                self.bitboards[3].set_bit_by_square(target);
                            }
                            Piece::Knight => {
                                self.bitboards[4].set_bit_by_square(target);
                            }
                            _ => return false,
                        }
                    },
                    Color::BLACK => {
                        // add new piece
                        match piece {
                            Piece::Queen => {
                                self.bitboards[7].set_bit_by_square(target);
                            },
                            Piece::Rook => {
                                self.bitboards[8].set_bit_by_square(target);
                            }
                            Piece::Bishop => {
                                self.bitboards[9].set_bit_by_square(target);
                            }
                            Piece::Knight => {
                                self.bitboards[10].set_bit_by_square(target);
                            }
                            _ => return false,
                        }
                    }
                }
            }
        }

        // if new en passant square not set, reset the field.
        if !new_en_passant {self.en_passant = Square::NO_SQUARE}

        // Figure out which piece moved and move it only if its not a promotion.
        match mv.get_special() {
            SpecialMove::Promotion(_) =>{},
            SpecialMove::PromotionCapture(_) => {},
            _ => {
                let origin_index = self.get_bitboard_index_at_square(origin)
                    .unwrap();
                self.bitboards[origin_index].move_bit_by_square(origin, target);

                // update castling rights if king or rooks are moved
                if origin_index == 2 { // white rooks
                    if origin == Square::A1 {
                        self.castling_rights &= 0b1011;        
                    } else if origin == Square::H1 { 
                        self.castling_rights &= 0b0111;        
                    }
                } else if origin_index == 8 { // black rooks
                    if origin == Square::A8 {
                        self.castling_rights &= 0b1110;        
                    } else if origin == Square::H8 {
                        self.castling_rights &= 0b1101;        
                    }
                } else if origin_index == 0 { // white king
                    self.castling_rights &= 0b0011;
                } else if origin_index == 6 { // black king
                    self.castling_rights &= 0b1100;
                }
            }
        }


        match self.to_move {
            Color::WHITE => {
                self.in_check = 
                    self.is_attacked(
                        self.bitboards[6]
                            .get_lsb_square()
                            .unwrap(),
                        Some(Color::WHITE))
            },
            Color::BLACK => {
                self.in_check = 
                    self.is_attacked(
                        self.bitboards[0]
                            .get_lsb_square()
                            .unwrap(), 
                        Some(Color::BLACK))
            },
        }


        // does move leave king in check?
        match self.to_move {
            Color::WHITE => {
                if self.is_attacked(
                    self.bitboards[0].get_lsb_square().unwrap(), 
                    Some(Color::BLACK)) {
                    
                    return false;
                }
            },
            Color::BLACK => {
                if self.is_attacked(
                    self.bitboards[6].get_lsb_square().unwrap(), 
                    Some(Color::WHITE)) {

                    return false;
                }
            }
        }

        // change color
        self.to_move = !self.to_move;
        true
    } 
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = ["K", "Q", "R", "B", "N", "P", "k", "q", "r", "b", "n", "p"];
        let mut display = String::new();
        for i in 0..64 {
            if i % 8 == 0 { display += &format!("\n{}  ", 8 - i/8) }
            let mut found = false;
            for (j, bb) in self.bitboards.iter().enumerate() {
                if bb.get_bit(i) == 1 {
                    display += chars[j];
                    found = true;
                }
            }
            if !found {display += ". "} else {display += " "}
        }
        display += "\n\n   a b c d e f g h";

        display += &format!(
            "\n\nto_move = {:?}\ncastling_rights = {:04b}\nen_passant = {}\n",
            self.to_move,
            self.castling_rights, 
            self.en_passant);
        write! { f, "{}", display }
    }
}
