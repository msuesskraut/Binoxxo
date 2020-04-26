//! Regression test for #1
//! Based on check_board example
use binoxxo::field::*;
use binoxxo::rules::{is_board_full, is_board_valid};
use std::str::FromStr;

#[test]
fn issue_1() {
    let ok = Board::from_str(
        "O O X O O X X O X X
         X O O X X O O X X O
         O X O X O X X O O X
         O X X O O X O X O X
         X O X O X O X O X O
         O X O X X O X X O O
         O X X O O X O O X X
         X O O X O X O X X O
         X O X O X O X O O X
         X X O X X O O X O O
        ",
    )
    .unwrap();

    assert!(is_board_full(&ok));
    assert!(!is_board_valid(&ok));
}
