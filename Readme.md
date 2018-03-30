[![Build Status](https://travis-ci.org/msuesskraut/Binoxxo.svg?branch=master)](https://travis-ci.org/msuesskraut/Binoxxo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](License.md)

# Binoxxo

Binoxxo is a library to create and check binoxxo puzzles.

## How to use

Add `binoxxo` to your dependencies.

See [examples](https://github.com/msuesskraut/Binoxxo/tree/master/examples)
and API documentation for details:

# Rules of Binoxxo

* there must be no empty fields
* there must be no more than two fields of the same token
  * either X O O X or O X X O
  * but not X O O O or X X X O
* each row and column must contain exactly the same numbers of X and O
* each row and column must be unique

For more details see:
[https://www.kreuzwortraetsel.ch/techniken-binoxxo/](https://www.kreuzwortraetsel.ch/techniken-binoxxo/)
in German.

# License

[MIT](License.md)
