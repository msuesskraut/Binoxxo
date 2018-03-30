#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate binoxxo;

use binoxxo::field::*;
use binoxxo::rules::is_board_valid;

fn main() {
    let ok = board!(4,
        X O X O
        O X O X
        X X O O
        O O X X
    );

    println!("Board {:?} is valid: {}", ok, is_board_valid(&ok));

    let wrong = board!(4,
        O X O X
        X O X O
        X O O O
        O X X O
    );

    println!("Board {:?} is valid: {}", wrong, is_board_valid(&wrong));
}
