# Binoxxo

[![Crates.io](https://img.shields.io/crates/v/binoxxo.svg)](https://crates.io/crates/binoxxo)
[![Docs](https://docs.rs/binoxxo/badge.svg)](https://docs.rs/binoxxo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](License.md)
[![Build and Test Status](https://github.com/msuesskraut/calc/workflows/Build%20and%20Test/badge.svg)](https://github.com/msuesskraut/calc/actions)

Binoxxo is a library to create and check binoxxo puzzles.

See webapp at: [https://msuesskraut.github.io/binoxxo/](https://msuesskraut.github.io/binoxxo/) for a demo.

## How to use

Add `binoxxo` to your dependencies.

See [examples](https://github.com/msuesskraut/Binoxxo/tree/master/examples)
and API documentation for details.

## Rules of Binoxxo

* there must be no empty fields
* there must be no more than two fields of the same token
  * either X O O X or O X X O
  * but not X O O O or X X X O
* each row and column must contain exactly the same numbers of X and O
* each row and column must be unique

For more details see:
[https://www.kreuzwortraetsel.ch/techniken-binoxxo/](https://www.kreuzwortraetsel.ch/techniken-binoxxo/)
in German.

## License

[MIT](License.md)
