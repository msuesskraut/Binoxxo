#![feature(stmt_expr_attributes)]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

#[macro_use]
pub mod field;
pub mod bruteforce;
