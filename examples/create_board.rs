//! This example demonstrates how to create a puzzle with `create_puzzle_board`.
//! It also shows how to print a board to terminal.

extern crate binoxxo;

/// Simple and platform independent wait-for-ENTER function.
fn pause() {
    use std::io::{stdin, Read};

    println!("Press ENTER to continue...");

    stdin().read(&mut [0u8]).unwrap();
}

fn main() {
    // create a puzzle
    let size = 10usize;
    let guesses = 15usize;
    let board = binoxxo::bruteforce::create_puzzle_board(size, guesses);

    println!("{}", board.to_string());

    // avoid closing the terminal
    pause();
}
