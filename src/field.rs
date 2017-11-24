#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Field {
    Empty,
    X,
    O,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    size: usize,
    fields: Vec<Field>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        assert!(0 == size % 2, "board size must be even");

        Board {
            size,
            fields: vec![Field::Empty; size * size],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, field: Field) {
        assert!(x < self.size);
        assert!(y < self.size);
        assert!(field != Field::Empty);

        self.fields[x * self.size + y] = field;
    }

    pub fn get(&self, x: usize, y: usize) -> Field {
        assert!(x < self.size);
        assert!(y < self.size);

        self.fields[x * self.size + y]
    }
}

#[allow(unused_macros)]
macro_rules! board {
    ($size : expr, $( $i : ident )* ) => {{
        use self::Field::X;
        use self::Field::O;
        use self::Field::Empty as E;
        let size = $size;
        let mut board = Board::new(size);
        let mut x = 0usize;
        let mut y = 0usize;
        $(
            match $i {
                X => board.set(x, y, X),
                O => board.set(x, y, O),
                _ => (),
            }
            x += 1;
            if size == x {
                #[allow(unused_assignments)]
                x = 0;
                #[allow(unused_assignments)]
                y += 1;
            }
        )*
        board
    }};
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
    fn build_board_with_macro() {
        let board = board!(2,
            X O
            E O
        );

        assert_eq!(2, board.size);
        assert_eq!(Field::X, board.get(0, 0));
        assert_eq!(Field::O, board.get(1, 0));
        assert_eq!(Field::Empty, board.get(0, 1));
        assert_eq!(Field::O, board.get(1, 1));
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

        assert_eq!(vec![Field::X,
                        Field::Empty,
                        Field::Empty,
                        Field::O],
                   board.fields);
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
}
