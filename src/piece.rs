// piece.rs

use std::fmt;
use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl Piece {

    pub fn from_index(i: u8) -> Option<Piece> {
        match i {
            0  => Some(Piece::King),
            1  => Some(Piece::Queen),
            2  => Some(Piece::Rook),
            3  => Some(Piece::Bishop),
            4  => Some(Piece::Knight),
            5  => Some(Piece::Pawn),
            6  => Some(Piece::King),
            7  => Some(Piece::Queen),
            8  => Some(Piece::Rook),
            9  => Some(Piece::Bishop),
            10 => Some(Piece::Knight),
            11 => Some(Piece::Pawn),
            _  => None
        }
    }
    
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_int(&self) -> u8 {
        *self as u8
    }

    pub fn as_str(&self, color: Color) -> &str {
        match color {
            Color::WHITE => {
                match self {
                    Piece::King   => "K",                 
                    Piece::Queen  => "Q",
                    Piece::Rook   => "R",
                    Piece::Bishop => "B",
                    Piece::Knight => "N",
                    Piece::Pawn   => "P",
                }
            },
            Color::BLACK => {
                match self {
                    Piece::King   => "k",                 
                    Piece::Queen  => "q",
                    Piece::Rook   => "r",
                    Piece::Bishop => "b",
                    Piece::Knight => "n",
                    Piece::Pawn   => "p",
                }

            }
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}",
            self.as_str(Color::BLACK)
        }
    }
}