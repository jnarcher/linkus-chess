// sliding_attacks.rs

use crate::bitboard::*;
use crate::square::Square;

/// Generate bishop rays at a square for the current board blockers.
pub fn get_bishop_rays(square: Square, blockers: Bitboard) -> Bitboard {
    let mut attacks = EMPTY;

    // SW
    let mut sq = square.clone();
    loop {
        sq = match sq.due_sw() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // SE
    let mut sq = square.clone();
    loop {
        sq = match sq.due_se() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // NE
    let mut sq = square.clone();
    loop {
        sq = match sq.due_ne() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // NW
    let mut sq = square.clone();
    loop {
        sq = match sq.due_nw() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    attacks
}


/// Generate rook rays at a square for the current board blockers.
pub fn get_rook_rays(square: Square, blockers: Bitboard) -> Bitboard {
    let mut attacks = EMPTY;

    // N
    let mut sq = square.clone();
    loop {
        sq = match sq.due_n() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // S
    let mut sq = square.clone();
    loop {
        sq = match sq.due_s() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // W
    let mut sq = square.clone();
    loop {
        sq = match sq.due_w() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    // E
    let mut sq = square.clone();
    loop {
        sq = match sq.due_e() {
            Some(s) => s,
            None => break
        };
        attacks |= Bitboard::from_square(sq);
        if blockers.get_bit_by_square(sq) == 1 { break }
    }

    attacks
}

#[inline]
pub fn get_queen_rays(square: Square, blockers: Bitboard) -> Bitboard {
    get_bishop_rays(square, blockers) | get_rook_rays(square, blockers)
}