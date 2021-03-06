//! This module implements a brute force random puzzle creator
//! for binoxxo.
//! See submodules for details.

pub mod build_board;
pub mod choose_move;
pub mod possible_move;
pub mod rules;

pub use self::build_board::create_full_board;
pub use self::build_board::create_puzzle_board;
