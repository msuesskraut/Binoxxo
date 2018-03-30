extern crate binoxxo;

use binoxxo::field::*;
use binoxxo::rules::is_board_valid;
use std::str::FromStr;

fn main() {
    let ok = Board::from_str(
        "X O X O 
        O X O X
        X X O O
        O O X X",
    ).unwrap();

    println!("Board {:?} is valid: {}", ok, is_board_valid(&ok));

    let wrong = Board::from_str(
        "O X O X
        X O X O
        X O O O
        O X X O",
    ).unwrap();

    println!("Board {:?} is valid: {}", wrong, is_board_valid(&wrong));
}
