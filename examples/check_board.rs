//! This example shows how to check an existing board.
extern crate binoxxo;

use binoxxo::field::*;
use binoxxo::rules::{is_board_valid, is_board_full};
use std::str::FromStr;
use std::string::ToString;

fn print_board_with_check(board: &Board)
{
    println!("Board:\n\n{} --> is full:  {}\n     is valid: {}",
    board.to_string(), is_board_full(board), is_board_valid(board));
}

fn main() {
    // the 1st board fulfills all binoxxo rules
    let ok = Board::from_str(
        "X O X O 
        O X O X
        X X O O
        O O X X",
    ).unwrap();

    print_board_with_check(&ok);
    println!("\n");

    // this board breaks two binoxxo rules:
    // - more than two adjacent O
    // - more O than X in a row
    let wrong = Board::from_str(
        "O X O X
        X O X O
        X O O O
        O X X O",
    ).unwrap();

    print_board_with_check(&wrong);
}
