use field::{Field, Board};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PossibleMove {
    NoMove,
    OneMove(Field),
    TwoMoves,
}

pub fn calc_possible_move(board: &Board, x: usize, y: usize) -> PossibleMove {
    if Field::Empty == board.get(x, y) {
        PossibleMove::TwoMoves
    } else {
        PossibleMove::NoMove
    }
}

fn is_valid_pair_rule(board: &Board, x: usize, y: usize) -> bool {
    let current = board.get(x, y);
    let size = board.get_size();
    assert!(Field::Empty != current);

    let three_up = y > 1 && current == board.get(x, y - 2) && current == board.get(x, y - 1);
    let three_down = y < size - 2 && current == board.get(x, y + 1) &&
                     current == board.get(x, y + 2);
    let three_middle = y > 0 && y < size - 1 && current == board.get(x, y - 1) &&
                       current == board.get(x, y + 1);

    let three_left = x > 1 && current == board.get(x - 2, y) && current == board.get(x - 1, y);
    let three_right = x < size - 2 && current == board.get(x + 1, y) &&
                      current == board.get(x + 2, y);
    let three_center = x > 0 && x < size - 1 && current == board.get(x - 1, y) &&
                       current == board.get(x + 1, y);

    !three_up && !three_down && !three_middle && !three_left && !three_right && !three_center
}

fn is_valid_colum(board: &Board, x: usize, y: usize) -> bool {
    let current = board.get(x, y);
    let mut count = 0;

    for y in 0..board.get_size() {
        if current == board.get(x, y) {
            count += 1;
        }
    }

    count <= board.get_size() / 2
}

fn is_valid_row(board: &Board, x: usize, y: usize) -> bool {
    let current = board.get(x, y);
    let mut count = 0;

    for x in 0..board.get_size() {
        if current == board.get(x, y) {
            count += 1;
        }
    }

    count <= board.get_size() / 2
}

fn calc_column_siganture(board: &Board, x: usize) -> Option<i64> {
    let mut sig = 0;
    let mut power_of_2 = 1;
    for y in 0..board.get_size() {
        match board.get(x, y) {
            Field::X => sig += power_of_2,
            Field::O => (),
            Field::Empty => return None,
        }
        power_of_2 *= 2;
    }
    return Some(sig);
}

fn is_unique_column(board: &Board, x: usize) -> bool {
    match calc_column_siganture(board, x) {
        Some(reference) => {
            (0..board.get_size())
                .filter(|col| *col != x)
                .map(|col| calc_column_siganture(board, col))
                .filter(|sig| *sig != Some(reference))
                .count() > 0
        }
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_not_possible_for_set_field() {
        let board = board!(2,
            X E
            E E
        );

        assert_eq!(PossibleMove::NoMove, calc_possible_move(&board, 0, 0));
    }

    #[test]
    fn two_options_for_empty_field() {
        let board = board!(2,
            E E
            E E
        );

        assert_eq!(PossibleMove::TwoMoves, calc_possible_move(&board, 0, 0));
    }

    #[test]
    fn x_with_adjacent_oo_is_valid() {
        let board = board!(4,
            O E E E
            O E E E
            X E E E
            E E E E
        );

        assert_eq!(true, is_valid_pair_rule(&board, 0, 2));
    }

    #[test]
    fn x_with_surounding_oo_is_valid() {
        let board = board!(4,
            X O X E
            E E E E
            E E E E
            E E E E
        );

        assert_eq!(true, is_valid_pair_rule(&board, 1, 0));
    }

    #[test]
    fn x_with_adjacent_xx_is_invalid_horizontally() {
        let board = board!(4,
            E E E E
            X X X E
            E E E E
            E E E E
        );

        assert_eq!(false, is_valid_pair_rule(&board, 0, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 2, 1));
    }

    #[test]
    fn x_with_adjacent_xx_is_invalid_vertically() {
        let board = board!(4,
            E O E E
            E O E E
            E O E E
            E E E E
        );

        assert_eq!(false, is_valid_pair_rule(&board, 1, 0));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_column_valid() {
        let board = board!(4,
            E X E E
            E X E E
            E O E E
            E O E E
        );

        assert_eq!(true, is_valid_colum(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_column_invalid() {
        let board = board!(4,
            E X E E
            E O E E
            E X E E
            E X E E
        );

        assert_eq!(false, is_valid_colum(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_row_valid() {
        let board = board!(4,
            E E E E
            O X X O
            E E E E
            E E E E
        );

        assert_eq!(true, is_valid_row(&board, 2, 1));
    }

    #[test]
    fn not_more_than_half_the_field_per_row_invalid() {
        let board = board!(4,
            E E E E
            O X O O
            E E E E
            E E E E
        );

        assert_eq!(false, is_valid_row(&board, 2, 1));
    }

    #[test]
    fn calc_column_siganture_of_non_empty_column() {
        let board = board!(4,
            O O X X
            O X X X
            O O O X
            O X O X
        );

        assert_eq!(Some(0), calc_column_siganture(&board, 0));
        assert_eq!(Some(10), calc_column_siganture(&board, 1));
        assert_eq!(Some(3), calc_column_siganture(&board, 2));
        assert_eq!(Some(15), calc_column_siganture(&board, 3));
    }

    #[test]
    fn calc_column_siganture_of_empty_column() {
        let board = board!(4,
            O O E E
            O X X E
            O O O E
            O X O E
        );

        assert_eq!(None, calc_column_siganture(&board, 2));
        assert_eq!(None, calc_column_siganture(&board, 3));
    }

    #[test]
    fn unique_column() {
        let board = board!(2,
            X O
            O X
        );

        assert_eq!(true, is_unique_column(&board, 0));
        assert_eq!(true, is_unique_column(&board, 1));
    }

    #[test]
    fn double_column() {
        let board = board!(2,
            X X
            O O
        );

        assert_eq!(false, is_unique_column(&board, 0));
        assert_eq!(false, is_unique_column(&board, 1));
    }

    #[test]
    fn empty_column_is_unique() {
        let board = board!(2,
            E E
            O O
        );

        assert_eq!(true, is_unique_column(&board, 0));
        assert_eq!(true, is_unique_column(&board, 1));
    }
}
