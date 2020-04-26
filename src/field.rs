//! The `field` module just models a board and its fields.
//! See:
//! - `enum` [`Field`](enum.Field.html)
//! - `struct` [`Board`](struct.Board.html)

use std::str::FromStr;
use std::string::ToString;

/// Represents on field of a binoxxo board.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Field {
    /// field is empty
    Empty,
    /// field contains a X
    X,
    /// field contains a O
    O,
}

/// A sqaure binoxxo board with a side length and its fields.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    size: usize,
    fields: Vec<Field>,
}

impl Board {
    /// Creates a new binoxxo board of side length `size`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is odd or `0`.
    pub fn new(size: usize) -> Board {
        assert!(size > 1, "board size must be larger than zero");
        assert!(0 == size % 2, "board size must be even");

        Board {
            size,
            fields: vec![Field::Empty; size * size],
        }
    }

    /// Sets field at column `x` and row `y` to `Empty`.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` are out-of-bounds (larger or equal to `get_size`).
    /// Panics if field at given coordinates is already `Empty`.
    pub fn clear(&mut self, x: usize, y: usize) {
        assert!(x < self.size);
        assert!(y < self.size);
        assert!(Field::Empty != self.fields[x * self.size + y]);

        self.fields[x * self.size + y] = Field::Empty;
    }

    /// Sets field at column `x` and row `y` to `field`.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` are out-of-bounds (larger or equal to `get_size`).
    /// Panics if `field` is `Empty`.
    pub fn set(&mut self, x: usize, y: usize, field: Field) {
        assert!(x < self.size);
        assert!(y < self.size);
        assert!(field != Field::Empty);

        self.fields[x * self.size + y] = field;
    }

    /// Returns field at column `x` and row `y` to `field`.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` are out-of-bounds (larger or equal to `get_size`).
    pub fn get(&self, x: usize, y: usize) -> Field {
        assert!(x < self.size);
        assert!(y < self.size);

        self.fields[x * self.size + y]
    }

    /// Returns side length of board `size`.
    pub fn get_size(&self) -> usize {
        self.size
    }
}

const X_STR: &str = "X";
const O_STR: &str = "O";
const EMPTY_STR: &str = "_";

impl FromStr for Board {
    type Err = String;

    fn from_str(b: &str) -> Result<Board, String> {
        let fields = b.split_whitespace().collect::<Vec<&str>>();
        let size = (fields.len() as f64).sqrt() as usize;
        if size * size != fields.len() {
            return Err("Number of string elements must be square number".to_string());
        }
        if size < 2 {
            return Err("Board too small (at least 2 elements)".to_string());
        }
        if 0 != size % 2 {
            return Err("Board size must be even".to_string());
        }
        let mut x = 0usize;
        let mut y = 0usize;
        let mut board = Board::new(size);
        for fieldstr in fields {
            let field;
            if fieldstr == X_STR {
                field = Field::X;
            } else if fieldstr == O_STR {
                field = Field::O;
            } else if fieldstr == EMPTY_STR {
                field = Field::Empty;
            } else {
                return Err("Unknown field string".to_string());
            }
            if Field::Empty != field {
                board.set(x, y, field);
            }
            x += 1;
            if x >= size {
                y += 1;
                x = 0;
            }
        }
        Ok(board)
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let size = self.get_size();
        for y in 0..size {
            for x in 0..size {
                match self.get(x, y) {
                    Field::X => result += X_STR,
                    Field::O => result += O_STR,
                    Field::Empty => result += EMPTY_STR,
                }
                if x < size - 1 {
                    result += " ";
                }
            }
            result += "\n";
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_new() {
        let board = Board::new(2);

        assert_eq!(2, board.size);
        assert_eq!(vec![Field::Empty; 4], board.fields);
    }

    #[test]
    fn build_from_str() {
        let board = Board::from_str(
            "X O
             _ O",
        )
        .unwrap();

        assert_eq!(2, board.size);
        assert_eq!(Field::X, board.get(0, 0));
        assert_eq!(Field::O, board.get(1, 0));
        assert_eq!(Field::O, board.get(1, 1));
        assert_eq!(Field::Empty, board.get(0, 1));
    }

    #[test]
    fn board_get_new() {
        let board = Board::new(2);

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(Field::Empty, board.get(x, y));
            }
        }
    }

    #[test]
    #[should_panic]
    fn board_get_x_oob() {
        let board = Board::new(2);

        board.get(2, 0);
    }

    #[test]
    #[should_panic]
    fn board_get_y_oob() {
        let board = Board::new(2);

        board.get(0, 2);
    }

    #[test]
    fn board_set() {
        let mut board = Board::new(2);

        board.set(0, 0, Field::X);
        board.set(1, 1, Field::O);

        assert_eq!(
            vec![Field::X, Field::Empty, Field::Empty, Field::O],
            board.fields
        );
    }

    #[test]
    fn board_set_get() {
        let mut board = Board::new(2);

        board.set(0, 0, Field::X);
        assert_eq!(Field::X, board.get(0, 0));
        board.set(1, 1, Field::O);
        assert_eq!(Field::O, board.get(1, 1));
        board.set(0, 1, Field::O);
        assert_eq!(Field::O, board.get(0, 1));
        board.set(1, 0, Field::X);
        assert_eq!(Field::X, board.get(1, 0));
    }

    #[test]
    #[should_panic]
    fn board_set_x_oob() {
        let mut board = Board::new(2);

        board.set(2, 0, Field::X);
    }

    #[test]
    #[should_panic]
    fn board_set_y_oob() {
        let mut board = Board::new(2);

        board.set(0, 2, Field::X);
    }

    #[test]
    #[should_panic]
    fn board_set_empty() {
        let mut board = Board::new(2);

        board.set(0, 0, Field::Empty);
    }

    #[test]
    fn clear_field() {
        let mut board = Board::from_str(
            "X O
            O X",
        )
        .unwrap();

        board.clear(0, 0);

        assert_eq!(Field::Empty, board.get(0, 0));
    }

    #[test]
    #[should_panic]
    fn clear_x_oob_panics() {
        let mut board = Board::from_str(
            "X O
            O X",
        )
        .unwrap();

        board.clear(2, 0);
    }

    #[test]
    #[should_panic]
    fn clear_y_oob_panics() {
        let mut board = Board::from_str(
            "X O
            O X",
        )
        .unwrap();

        board.clear(0, 2);
    }

    #[test]
    #[should_panic]
    fn clear_already_empty_field_panics() {
        let mut board = Board::from_str(
            "_ O
            O X",
        )
        .unwrap();

        board.clear(0, 0);
    }

    #[test]
    fn board_to_string() {
        let board = Board::from_str(
            "
            _ O
            O X
        ",
        )
        .unwrap();

        assert_eq!("_ O\nO X\n".to_string(), board.to_string());
    }
}
