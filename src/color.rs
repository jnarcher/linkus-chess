// color.rs

use std::ops::Not;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}
