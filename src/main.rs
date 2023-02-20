// main.rs

use crate::square::Square;
pub mod square;

fn main() {
    let sq = Square::D4;
    println!("{}", sq);
}
