// bitboard.rs

use std::ops;
use std::fmt;
use crate::square::Square;

pub const EMPTY: Bitboard = Bitboard(0);


#[derive(Clone, Copy, Debug)]
pub struct Bitboard(u64);

impl Bitboard {

    pub const EMPTY: u64 = 0;

    /// Create new Bitboard from u64.
    pub fn new(int: u64) -> Bitboard {
        Bitboard(int)
    }

    /// Create new empty Bitboard.
    pub fn empty() -> Bitboard {
        Bitboard(0) 
    }

    /// Create Bitboard with the bit at the given square set.
    pub fn from_square(square: Square) -> Bitboard {
        Bitboard(1u64 << square.to_int())
    }

    /// Get integer representation.
    #[inline] 
    pub fn to_int(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Get bit by index.
    pub fn get_bit(&self, index: u8) -> u8 {
        match self.0 & (1 << index) {
            0 => 0,
            _ => 1,
        }
    }

    /// Get bit at square.
    pub fn get_bit_by_square(&self, square: Square) -> u8 {
        match self.0 & (1 << square.to_index() as u64) {
            0 => 0,
            _ => 1,
        }
    }

    /// Set bit at index to 1.
    pub fn set_bit(&mut self, index: u8) {
        *self |= 1u64 << index
    }

    /// Set bit at square to 1.
    pub fn set_bit_by_square(&mut self, square: Square) {
        *self |= 1u64 << square.to_int()
    }

    /// Set bit at index to 0.
    pub fn pop_bit(&mut self, index: u8) {
        if self.get_bit(index) == 1 {
            *self ^= 1u64 << index;
        }
    }

    /// Set bit at square to 0.
    pub fn pop_bit_by_square(&mut self, square: Square) {
        if self.get_bit_by_square(square) == 1 {
            *self ^= 1u64 << square.to_int();
        }
    }

    /// If the bit indexed by source is set and the bit indexed by target is 
    /// not, swap the bits.
    pub fn move_bit(&mut self, source: u8, target: u8) {
        if self.get_bit(source) == 1 && self.get_bit(target) == 0 {
            self.pop_bit(source);
            self.set_bit(target);
        }
    } 

    /// If the bit at the source square is set and the bit at the target square 
    /// is not, swap the bits.
    pub fn move_bit_by_square(&mut self, source: Square, target: Square) {
        if 
            self.get_bit_by_square(source) == 1 && 
            self.get_bit_by_square(target) == 0

        {
            self.pop_bit_by_square(source);
            self.set_bit_by_square(target);
        }
    } 

    /// Count number of set bits.
    pub fn count_bits(&self) -> u8 {
        let mut count = 0;
        let mut bb = self.to_int();
        while bb != 0 {
            bb &= bb - 1;
            count += 1;
        }
        count
    }

    /// Get least significant bit index.
    pub fn get_lsb_index(&self) -> Option<u8> {
        if self.to_int() != 0 {
            Some( 
                ( ((!self + 1) & self) - 1 )
                .count_bits() 
            )
        } else {
            None
        }
    }

    /// Get least significant bit index.
    pub fn get_lsb_square(&self) -> Option<Square> {
        if self.to_int() != 0 {
            let index = (((!self + 1) & self) - 1 ).count_bits();
            Some(Square::new(index))
        } else {
            None
        }
    }

}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for i in 0..64 {
            if i % 8 ==  0 {
                display += &format!("\n{}  ", 8 - i/8);
            }
            display += match self.get_bit(i) {
                0 => ".",
                1 => "1",
                _ => panic!("Bitboard `get_bit()` faliure."),
            };
            display += " ";
        }
        display += "\n\n   a b c d e f g h";

        display += &format!("\n\ndec: {}\nhex: {:x}", self.to_int(), self.to_int());

        write! { f, "{}", display }
    }
}

//~~~~~~~~~~~~~~~~  ADD  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::Add<Bitboard> for Bitboard {
    type Output = Self;
    fn add(self, other: Bitboard) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl ops::Add<u64> for Bitboard {
    type Output = Self;
    fn add(self, other: u64) -> Self::Output {
        Self(self.0 + other)
    }
}


//~~~~~~~~~~~~~~~~  SUB  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::Sub<Bitboard> for Bitboard {
    type Output = Self;
    fn sub(self, other: Bitboard) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl ops::Sub<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn sub(self, other: Bitboard) -> Self::Output {
        Bitboard(self.0 - other.0)
    }
}

impl ops::Sub<u64> for Bitboard {
    type Output = Self;
    fn sub(self, other: u64) -> Self::Output {
        Self(self.0 - other)
    }
}

impl ops::Sub<u64> for &Bitboard {
    type Output = Bitboard;
    fn sub(self, other: u64) -> Self::Output {
        Bitboard(self.0 - other)
    }
}

//~~~~~~~~~~~~~~~~  NOT  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl ops::Not for &Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}


//~~~~~~~~~~~~~~~~  AND  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::BitAnd<Bitboard> for Bitboard {
    type Output = Self;
    fn bitand(self, other: Bitboard) -> Self::Output {
        Self(self.0 & other.0)
    }
} 

impl ops::BitAnd<&Bitboard> for Bitboard {
    type Output = Self;
    fn bitand(self, other: &Bitboard) -> Self::Output {
        Self(self.0 & other.0)
    }
} 

impl ops::BitAnd<u64> for  Bitboard{
    type Output = Self;
    fn bitand(self, other: u64) -> Self::Output {
        Self(self.0 & other)
    }
} 

impl ops::BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.0 &= other.0;
    }
} 

impl ops::BitAndAssign<&Bitboard> for Bitboard {
    fn bitand_assign(&mut self, other: &Bitboard) {
        self.0 &= other.0;
    }
} 

impl ops::BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, other: u64) {
        self.0 &= other;
    }
} 

//~~~~~~~~~~~~~~~~  OR  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::BitOr<Bitboard> for Bitboard {
    type Output = Self;
    fn bitor(self, other: Bitboard) -> Self::Output {
        Self(self.0 | other.0)
    }
} 

impl ops::BitOr<u64> for  Bitboard{
    type Output = Self;
    fn bitor(self, other: u64) -> Self::Output {
        Self(self.0 | other)
    }
} 

impl ops::BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.0 |= other.0;
    }
} 

impl ops::BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, other: &Bitboard) {
        self.0 |= other.0;
    }
} 

impl ops::BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, other: u64) {
        self.0 |= other;
    }
} 

//~~~~~~~~~~~~~~~~  XOR  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ops::BitXor<Bitboard> for Bitboard {
    type Output = Self;
    fn bitxor(self, other: Bitboard) -> Self::Output {
        Self(self.0 ^ other.0)
    }
} 

impl ops::BitXor<u64> for  Bitboard{
    type Output = Self;
    fn bitxor(self, other: u64) -> Self::Output {
        Self(self.0 ^ other)
    }
} 

impl ops::BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, other: Bitboard) {
        self.0 ^= other.0;
    }
} 

impl ops::BitXorAssign<&Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, other: &Bitboard) {
        self.0 ^= other.0;
    }
} 

impl ops::BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, other: u64) {
        self.0 ^= other;
    }
} 