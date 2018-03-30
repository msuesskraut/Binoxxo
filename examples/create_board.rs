//! This example demonstrates how to create a puzzle with `create_puzzle_board`.
//! It also shows how to print a board to terminal.

extern crate binoxxo;

use binoxxo::field::Field;

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

    // print puzzle to stdout
    for y in 0..size {
        let mut line = String::new();
        for x in 0..size {
            match board.get(x, y) {
                Field::X => line += "X ",
                Field::O => line += "O ",
                Field::Empty => line += "_ ",
            }
        }
        println!("{}", line);
    }

    // avoid closing the terminal
    pause();
}
