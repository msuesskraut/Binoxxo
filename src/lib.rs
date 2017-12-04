#![feature(stmt_expr_attributes)]

extern crate rand;
#[cfg(test)]
extern crate quickcheck;

#[macro_use]
pub mod field;
pub mod bruteforce;
