// move_tables.rs

use crate::bitboard::*;

pub static mut PAWN_PUSHES: [[Bitboard; 64]; 2] = [[EMPTY; 64]; 2];
pub static mut PAWN_DOUBLE_PUSHES: [[Bitboard; 64]; 2] = [[EMPTY; 64]; 2];
pub static mut PAWN_ATTACKS: [[Bitboard; 64]; 2] = [[EMPTY; 64]; 2];

pub static mut KNIGHT_ATTACKS: [Bitboard; 64] = [EMPTY; 64];
pub static mut KING_ATTACKS: [Bitboard; 64]= [EMPTY; 64];


pub const NOT_A_FILE: u64 = 18374403900871474942;
pub const NOT_H_FILE: u64 = 9187201950435737471; 
pub const NOT_AB_FILE: u64 = 18229723555195321596;
pub const NOT_GH_FILE: u64 = 4557430888798830399;


fn gen_pawn_pushes() {
    for i in 0..64 {
        let loc = 1u64 << i;
        unsafe {
            PAWN_PUSHES[0][i] |= loc >> 8; 
            PAWN_PUSHES[1][i] |= loc << 8;
        }
    }
}

fn gen_pawn_double_pushes() {
    for i in 0..64 {
        let loc = 1u64 << i;

        if i/8 == 6 {
            unsafe {
                PAWN_DOUBLE_PUSHES[0][i] |= loc >> 16; 
            }
        } else if i/8 == 1 {
            unsafe {
                PAWN_DOUBLE_PUSHES[1][i] |= loc << 16; 
            }
        }
    }
}

fn gen_pawn_attacks() {
    for i in 0..64 {
        let loc = 1u64 << i;
        unsafe {
            PAWN_ATTACKS[0][i] |= loc >> 7 & NOT_A_FILE;
            PAWN_ATTACKS[0][i] |= loc >> 9 & NOT_H_FILE;
            PAWN_ATTACKS[1][i] |= loc << 7 & NOT_H_FILE;
            PAWN_ATTACKS[1][i] |= loc << 9 & NOT_A_FILE;
        }
    }
}

fn gen_knight_attacks() {
    for i in 0..64 {
        let loc = 1u64 << i;
        unsafe{
            KNIGHT_ATTACKS[i] |= loc >> 15 & NOT_A_FILE;
            KNIGHT_ATTACKS[i] |= loc >> 17 & NOT_H_FILE;
            KNIGHT_ATTACKS[i] |= loc >> 6 & NOT_AB_FILE;
            KNIGHT_ATTACKS[i] |= loc >> 10 & NOT_GH_FILE;
            KNIGHT_ATTACKS[i] |= loc << 15 & NOT_H_FILE;
            KNIGHT_ATTACKS[i] |= loc << 17 & NOT_A_FILE;
            KNIGHT_ATTACKS[i] |= loc << 6 & NOT_GH_FILE;
            KNIGHT_ATTACKS[i] |= loc << 10 & NOT_AB_FILE;
        }
    }
}

fn gen_king_attacks() {
    for i in 0..64 {
        let loc = 1u64 << i;
        unsafe {
            KING_ATTACKS[i] |= loc >> 7 & NOT_A_FILE;
            KING_ATTACKS[i] |= loc >> 8;
            KING_ATTACKS[i] |= loc >> 9 & NOT_H_FILE;
            KING_ATTACKS[i] |= loc >> 1 & NOT_H_FILE;
            KING_ATTACKS[i] |= loc << 1 & NOT_A_FILE;
            KING_ATTACKS[i] |= loc << 7 & NOT_H_FILE;
            KING_ATTACKS[i] |= loc << 8;
            KING_ATTACKS[i] |= loc << 9 & NOT_A_FILE;
        }
    }
}

pub fn gen_tables() {
    gen_pawn_pushes();
    gen_pawn_double_pushes();
    gen_pawn_attacks();
    gen_knight_attacks();
    gen_king_attacks();
}