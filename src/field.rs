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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_new() {
        let board = Board::new(3);

        assert_eq!(3, board.size);
        assert_eq!(vec![Field::Empty; 9], board.fields);
    }

    #[test]
    fn board_get_new() {
        let board = Board::new(3);

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(Field::Empty, board.get(x, y));
            }
        }
    }

    #[test]
    #[should_panic]
    fn board_get_x_oob() {
        let board = Board::new(3);

        board.get(3, 0);
    }

    #[test]
    #[should_panic]
    fn board_get_y_oob() {
        let board = Board::new(3);

        board.get(0, 3);
    }

    #[test]
    fn board_set() {
        let mut board = Board::new(3);

        board.set(0, 0, Field::X);
        board.set(1, 1, Field::O);
        board.set(2, 2, Field::X);
        board.set(0, 2, Field::O);
        board.set(2, 0, Field::X);

        assert_eq!(vec![Field::X,
                        Field::Empty,
                        Field::O,
                        Field::Empty,
                        Field::O,
                        Field::Empty,
                        Field::X,
                        Field::Empty,
                        Field::X],
                   board.fields);
    }

    #[test]
    fn board_set_get() {
        let mut board = Board::new(3);

        board.set(0, 0, Field::X);
        assert_eq!(Field::X, board.get(0, 0));
        board.set(1, 1, Field::O);
        assert_eq!(Field::O, board.get(1, 1));
        board.set(2, 2, Field::X);
        assert_eq!(Field::X, board.get(2, 2));
        board.set(0, 2, Field::O);
        assert_eq!(Field::O, board.get(0, 2));
        board.set(2, 0, Field::X);
        assert_eq!(Field::X, board.get(2, 0));
    }

    #[test]
    #[should_panic]
    fn board_set_x_oob() {
        let mut board = Board::new(3);

        board.set(3, 0, Field::X);
    }

    #[test]
    #[should_panic]
    fn board_set_y_oob() {
        let mut board = Board::new(3);

        board.set(0, 3, Field::X);
    }

    #[test]
    #[should_panic]
    fn board_set_empty() {
        let mut board = Board::new(3);

        board.set(0, 0, Field::Empty);
    }
}
