#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate binoxxo;

use binoxxo::field::*;

fn main() {
    let _ok = board!(6,
        X O X O X O
        O X O X O X
        X X O O X X
        O O X X O O
        O O X O X X
    );

    let _wrong = board!(6,
        X O X O X O
        O X O X O X
        X X O O X X
        O O X X O O
        O O X O X X
    );
}
