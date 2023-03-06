// square.rs

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(u8);

impl Square {

    /// Creates a new square at the given board index.
    pub fn new(index: u8) -> Square {
        Square(index)
    }

    /// Create square from agebraic notation
    pub fn from_alg(coord: &str) -> Option<Square> {
        let mut chars = coord.chars();
        let file = match chars.next() {
            Some(inner) => inner as u32,
            None => return None,
        };
        let rank = match chars.next() {
            Some(inner) => inner as u32,
            None => return None,
        };


        let a = 'a' as u32;
        let h = 'h' as u32;
        let ascii_1 = '1' as u32;
        let ascii_8 = '8' as u32;

        if file >= a && file <= h {
            if rank >= ascii_1 && rank <= ascii_8 {
                return Some(Square::new(((7 - (rank - ascii_1))*8 + (file - a )) as u8));
            }
        }
        None
    }

    /// Gets the square index as usize.
    #[inline] 
    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    /// Gets the square index as u8.
    #[inline] 
    pub fn to_int(&self) -> u8 {
        self.0 as u8
    } 

    /// Gets the square file. 
    #[inline] 
    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    /// Gets the square rank.
    #[inline] 
    pub fn rank(&self) -> u8 {
        8 - (self.0 / 8)
    }

    /// Gets the square one rank above. Returns none if current square is on the edge.
    #[inline]
    pub fn due_n(&self) -> Option<Square> {
        if self.rank() == 8 {
            return None;
        }
        return Some(Square::new(self.0 - 8))
    }

    /// Gets the square one rank below. Returns none if current square is on the edge.
    #[inline]
    pub fn due_s(&self) -> Option<Square> {
        if self.rank() == 1 {
            return None;
        }
        return Some(Square::new(self.0 + 8))
    }

    /// Gets the square one file to the left. Returns none if current square is 
    /// on the edge.
    #[inline]
    pub fn due_w(&self) -> Option<Square> {
        if self.file() == 0 {
            return None;
        }
        return Some(Square::new(self.0 - 1))
    }

    /// Gets the square one file to the right. Returns none if current square 
    /// is on the edge.
    #[inline]
    pub fn due_e(&self) -> Option<Square> {
        if self.file() == 7 {
            return None;
        }
        return Some(Square::new(self.0 + 1))
    }

    /// Gets the square diagonally north west. Returns none if current square 
    /// is on the edge.
    #[inline]
    pub fn due_nw(&self) -> Option<Square> {
        if self.file() == 0 || self.rank() == 8 {
            return None;
        }
        return Some(Square::new(self.0 - 9))
    }

    /// Gets the square diagonally north east. Returns none if current square 
    /// is on the edge.
    #[inline]
    pub fn due_ne(&self) -> Option<Square> {
        if self.file() == 7 || self.rank() == 8 {
            return None;
        }
        return Some(Square::new(self.0 - 7))
    }

    /// Gets the square diagonally south west. Returns none if current square 
    /// is on the edge.
    #[inline]
    pub fn due_sw(&self) -> Option<Square> {
        if self.file() == 0 || self.rank() == 1 {
            return None;
        }
        return Some(Square::new(self.0 + 7))
    }

    /// Gets the square diagonally south east. Returns none if current square is on the edge.
    #[inline]
    pub fn due_se(&self) -> Option<Square> {
        if self.file() == 7 || self.rank() == 1 {
            return None;
        }
        return Some(Square::new(self.0 + 9))
    }

    // Quick square lookup.
    pub const A8: Square = Square(0);
    pub const B8: Square = Square(1);
    pub const C8: Square = Square(2);
    pub const D8: Square = Square(3);
    pub const E8: Square = Square(4);
    pub const F8: Square = Square(5);
    pub const G8: Square = Square(6);
    pub const H8: Square = Square(7);
    pub const A7: Square = Square(8);
    pub const B7: Square = Square(9);
    pub const C7: Square = Square(10);
    pub const D7: Square = Square(11);
    pub const E7: Square = Square(12);
    pub const F7: Square = Square(13);
    pub const G7: Square = Square(14);
    pub const H7: Square = Square(15);
    pub const A6: Square = Square(16);
    pub const B6: Square = Square(17);
    pub const C6: Square = Square(18);
    pub const D6: Square = Square(19);
    pub const E6: Square = Square(20);
    pub const F6: Square = Square(21);
    pub const G6: Square = Square(22);
    pub const H6: Square = Square(23);
    pub const A5: Square = Square(24);
    pub const B5: Square = Square(25);
    pub const C5: Square = Square(26);
    pub const D5: Square = Square(27);
    pub const E5: Square = Square(28);
    pub const F5: Square = Square(29);
    pub const G5: Square = Square(30);
    pub const H5: Square = Square(31);
    pub const A4: Square = Square(32);
    pub const B4: Square = Square(33);
    pub const C4: Square = Square(34);
    pub const D4: Square = Square(35);
    pub const E4: Square = Square(36);
    pub const F4: Square = Square(37);
    pub const G4: Square = Square(38);
    pub const H4: Square = Square(39);
    pub const A3: Square = Square(40);
    pub const B3: Square = Square(41);
    pub const C3: Square = Square(42);
    pub const D3: Square = Square(43);
    pub const E3: Square = Square(44);
    pub const F3: Square = Square(45);
    pub const G3: Square = Square(46);
    pub const H3: Square = Square(47);
    pub const A2: Square = Square(48);
    pub const B2: Square = Square(49);
    pub const C2: Square = Square(50);
    pub const D2: Square = Square(51);
    pub const E2: Square = Square(52);
    pub const F2: Square = Square(53);
    pub const G2: Square = Square(54);
    pub const H2: Square = Square(55);
    pub const A1: Square = Square(56);
    pub const B1: Square = Square(57);
    pub const C1: Square = Square(58);
    pub const D1: Square = Square(59);
    pub const E1: Square = Square(60);
    pub const F1: Square = Square(61);
    pub const G1: Square = Square(62);
    pub const H1: Square = Square(63);
    pub const NO_SQUARE: Square = Square(64);
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 64  {
            write! {
                f,
                "NONE",
            }
        } else {
            write! {
                f,
                "{}{}",
                (('a' as u8) + self.file()) as char,
                (('0' as u8) + self.rank()) as char,
            }
        }
    }
}

// list of all squares
pub const ALL_SQUARES: [Square; 64] = [
    Square(0),
    Square(1),
    Square(2),
    Square(3),
    Square(4),
    Square(5),
    Square(6),
    Square(7),
    Square(8),
    Square(9),
    Square(10),
    Square(11),
    Square(12),
    Square(13),
    Square(14),
    Square(15),
    Square(16),
    Square(17),
    Square(18),
    Square(19),
    Square(20),
    Square(21),
    Square(22),
    Square(23),
    Square(24),
    Square(25),
    Square(26),
    Square(27),
    Square(28),
    Square(29),
    Square(30),
    Square(31),
    Square(32),
    Square(33),
    Square(34),
    Square(35),
    Square(36),
    Square(37),
    Square(38),
    Square(39),
    Square(40),
    Square(41),
    Square(42),
    Square(43),
    Square(44),
    Square(45),
    Square(46),
    Square(47),
    Square(48),
    Square(49),
    Square(50),
    Square(51),
    Square(52),
    Square(53),
    Square(54),
    Square(55),
    Square(56),
    Square(57),
    Square(58),
    Square(59),
    Square(60),
    Square(61),
    Square(62),
    Square(63),
];
