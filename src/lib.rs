//! Binoxxo is a library to create and check binoxxo puzzles.
//!
//! # How to use
//!
//! Add `binoxxo` to your dependencies.
//!
//! # Create a puzzle
//!
//! A binoxxo puzzle is a square board with either X or O or empty fields.
//! To fill the board the puzzler must fill the empty fields accourding
//! to the binoxxo rules.
//!
//! ## Board
//!
//! You can create a board with the [`Board::FromStr`](field/struct.Board.html).
//!
//! ```
//! use binoxxo::field::{Board, Field};
//! use std::str::FromStr;
//! let board = Board::from_str("
//!     X O X O
//!     O X O X
//!     X X O O
//!     O O X X"
//! ).unwrap();
//! ```
//! It creates a [`Board`](field/struct.Board.html) struct. You can also create `Board`s
//! manually.
//!
//! ## create_puzzle_board
//!
//! Use [`create_puzzle_board`](bruteforce/Board::from_str/fn.create_puzzle_board.html) to create a random puzzle:
//! ```
//! let size = 10usize;
//! let guesses = 15usize;
//! let board = binoxxo::bruteforce::create_puzzle_board(size, guesses);
//! println!("Board:\n{}", board.to_string());
//! ```
//! The `size` is length of one square side in number of fields.
//! 10 is a common size.
//!
//! `guesses` is an tuner for the difficulty of the resulting puzzle.
//! The larger `guesses` the more complicated the resulting puzzle and
//! the more empty fields does the board has.
//!
//! ## create_full_board
//!
//! You can also create a randomly full board without empty fields:
//! ```
//! let size = 10usize;
//! let board = binoxxo::bruteforce::create_full_board(size);
//! println!("Board:\n{}", board.to_string());
//! ```
//!
//! See the create_board example for how to print the resulting board.
//!
//! # Check a board
//!
//! You can check a full board with the [`is_valid_board`](bruteforce/rules/fn.is_board_valid.html):
//! ```
//! use binoxxo::field::Board;
//! use binoxxo::rules::is_board_valid;
//! use std::str::FromStr;
//! let board = Board::from_str("
//!     X O X O
//!     O X O X
//!     X X O O
//!     O O X X"
//! ).unwrap();
//! println!("Board is valid: {}", is_board_valid(&board));
//! ```
//! # Rules
//!
//! * there must be no empty fields
//! * there must be no more than two fields of the same token
//!   * either X O O X or O X X O
//!   * but not X O O O or X X X O
//! * each row and column must contain exactly the same numbers of X and O
//! * each row and column must be unique
//!
//! For more details see:
//! [https://www.kreuzwortraetsel.ch/techniken-binoxxo/](https://www.kreuzwortraetsel.ch/techniken-binoxxo/)
//! in German.
//!
//! # License
//!
//! The crate is published under the [MIT](https://opensource.org/licenses/MIT) license.

extern crate rand;

pub mod bruteforce;
pub mod field;
pub use bruteforce::rules;
