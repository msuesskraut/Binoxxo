//! This module can check entiere boards or single moves, whether the fullfill
//! the binoxxo rules.

use std::collections::HashSet;

use field::{Board, Field};

fn is_valid_pair_rule(board: &Board, x: usize, y: usize) -> bool {
    let current = board.get(x, y);
    let size = board.get_size();
    assert!(Field::Empty != current);

    let three_up = y > 1 && current == board.get(x, y - 2) && current == board.get(x, y - 1);
    let three_down =
        y < size - 2 && current == board.get(x, y + 1) && current == board.get(x, y + 2);
    let three_middle =
        y > 0 && y < size - 1 && current == board.get(x, y - 1) && current == board.get(x, y + 1);

    let three_left = x > 1 && current == board.get(x - 2, y) && current == board.get(x - 1, y);
    let three_right =
        x < size - 2 && current == board.get(x + 1, y) && current == board.get(x + 2, y);
    let three_center =
        x > 0 && x < size - 1 && current == board.get(x - 1, y) && current == board.get(x + 1, y);

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
    Some(sig)
}

fn is_unique_column(board: &Board, x: usize) -> bool {
    match calc_column_siganture(board, x) {
        Some(reference) => {
            (0..board.get_size())
                .filter(|col| *col != x)
                .map(|col| calc_column_siganture(board, col))
                .filter(|sig| *sig == Some(reference))
                .count() == 0
        }
        None => true,
    }
}

fn calc_row_siganture(board: &Board, y: usize) -> Option<i64> {
    let mut sig = 0;
    let mut power_of_2 = 1;
    for x in 0..board.get_size() {
        match board.get(x, y) {
            Field::X => sig += power_of_2,
            Field::O => (),
            Field::Empty => return None,
        }
        power_of_2 *= 2;
    }
    Some(sig)
}

fn is_unique_row(board: &Board, y: usize) -> bool {
    match calc_row_siganture(board, y) {
        Some(reference) => {
            (0..board.get_size())
                .filter(|row| *row != y)
                .map(|row| calc_row_siganture(board, row))
                .filter(|sig| *sig == Some(reference))
                .count() == 0
        }
        None => true,
    }
}

/// Returns whether the content of the field at column `x` and row `y`
/// on board `board` fullfills the binoxxo rules.
/// The rest of the board is *not* checked, and, hence, even if
/// this function returns `true`, the entiere board `board` may be invalid.
///
/// To check the entiere board use: `fn` [`is_board_valid`](fn.is_board_valid.html).
pub fn is_move_valid(board: &Board, x: usize, y: usize) -> bool {
    is_valid_pair_rule(board, x, y) && is_valid_colum(board, x, y) && is_valid_row(board, x, y)
        && is_unique_row(board, y) && is_unique_column(board, x)
}

fn are_columns_unique(board: &Board) -> bool {
    let column_sigs = (0..board.get_size())
        .map(|col| calc_column_siganture(board, col))
        .collect::<HashSet<Option<i64>>>();
    column_sigs.len() == board.get_size()
}

fn are_rows_unique(board: &Board) -> bool {
    let row_sigs = (0..board.get_size())
        .map(|row| calc_row_siganture(board, row))
        .collect::<HashSet<Option<i64>>>();
    row_sigs.len() == board.get_size()
}

fn are_rows_balanced(board: &Board) -> bool {
    let size = board.get_size();
    let half_size = size / 2;
    let get_num_fields =
        |field: Field, y: usize| (0..size).filter(|x| field == board.get(*x, y)).count();
    (0..size)
        .map(|y| (get_num_fields(Field::X, y), get_num_fields(Field::O, y)))
        .all(|num_xo| (half_size, half_size) == num_xo)
}

fn are_columns_balanced(board: &Board) -> bool {
    let size = board.get_size();
    let half_size = size / 2;
    let get_num_fields =
        |field: Field, x: usize| (0..size).filter(|y| field == board.get(x, *y)).count();
    (0..size)
        .map(|x| (get_num_fields(Field::X, x), get_num_fields(Field::O, x)))
        .all(|num_xo| (half_size, half_size) == num_xo)
}

fn has_no_more_than_two_same_neightbors(board: &Board) -> bool {
    let size = board.get_size();
    for x in 1..(size - 1) {
        for y in 1..(size - 1) {
            let f = board.get(x, y);
            if ((f == board.get(x - 1, y)) && (f == board.get(x + 1, y)))
                || ((f == board.get(x, y - 1)) && (f == board.get(x, y + 1)))
            {
                return false;
            }
        }
    }
    true
}

/// Returns whether all fields on the board `board` fullfill
/// the binoxxo rules.
///
/// To check a single move use: `fn` [`is_move_valid`](fn.is_move_valid.html).
pub fn is_board_valid(board: &Board) -> bool {
    are_columns_unique(board) && are_rows_unique(board) && are_rows_balanced(board)
        && are_columns_balanced(board) && has_no_more_than_two_same_neightbors(board)
}

/// Returns whether the board is full (i.e. has no `Empty` fields).
pub fn is_board_full(board: &Board) -> bool {
    let size = board.get_size();
    for x in 1..(size - 1) {
        for y in 1..(size - 1) {
            if Field::Empty == board.get(x, y) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn x_with_adjacent_oo_is_valid() {
        let board = Board::from_str(
            "O _ _ _
            O _ _ _
            X _ _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(true, is_valid_pair_rule(&board, 0, 2));
    }

    #[test]
    fn x_with_surounding_oo_is_valid() {
        let board = Board::from_str(
            "X O X _
            _ _ _ _
            _ _ _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(true, is_valid_pair_rule(&board, 1, 0));
    }

    #[test]
    fn x_with_adjacent_xx_is_invalid_horizontally() {
        let board = Board::from_str(
            "_ _ _ _
            X X X _
            _ _ _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(false, is_valid_pair_rule(&board, 0, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 2, 1));
    }

    #[test]
    fn x_with_adjacent_xx_is_invalid_vertically() {
        let board = Board::from_str(
            "_ O _ _
            _ O _ _
            _ O _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(false, is_valid_pair_rule(&board, 1, 0));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 1));
        assert_eq!(false, is_valid_pair_rule(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_column_valid() {
        let board = Board::from_str(
            "_ X _ _
            _ X _ _
            _ O _ _
            _ O _ _",
        ).unwrap();

        assert_eq!(true, is_valid_colum(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_column_invalid() {
        let board = Board::from_str(
            "_ X _ _
            _ O _ _
            _ X _ _
            _ X _ _",
        ).unwrap();

        assert_eq!(false, is_valid_colum(&board, 1, 2));
    }

    #[test]
    fn not_more_than_half_the_field_per_row_valid() {
        let board = Board::from_str(
            "
            _ _ _ _
            O X X O
            _ _ _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(true, is_valid_row(&board, 2, 1));
    }

    #[test]
    fn not_more_than_half_the_field_per_row_invalid() {
        let board = Board::from_str(
            "
            _ _ _ _
            O X O O
            _ _ _ _
            _ _ _ _",
        ).unwrap();

        assert_eq!(false, is_valid_row(&board, 2, 1));
    }

    #[test]
    fn calc_column_siganture_of_non_empty_column() {
        let board = Board::from_str(
            "
            O O X X
            O X X X
            O O O X
            O X O X",
        ).unwrap();

        assert_eq!(Some(0), calc_column_siganture(&board, 0));
        assert_eq!(Some(10), calc_column_siganture(&board, 1));
        assert_eq!(Some(3), calc_column_siganture(&board, 2));
        assert_eq!(Some(15), calc_column_siganture(&board, 3));
    }

    #[test]
    fn calc_column_siganture_of_empty_column() {
        let board = Board::from_str(
            "
            O O _ _
            O X X _
            O O O _
            O X O _",
        ).unwrap();

        assert_eq!(None, calc_column_siganture(&board, 2));
        assert_eq!(None, calc_column_siganture(&board, 3));
    }

    #[test]
    fn unique_column() {
        let board = Board::from_str(
            "
            X O
            O X",
        ).unwrap();

        assert_eq!(true, is_unique_column(&board, 0));
        assert_eq!(true, is_unique_column(&board, 1));
    }

    #[test]
    fn double_column() {
        let board = Board::from_str(
            "
            X X
            O O",
        ).unwrap();

        assert_eq!(false, is_unique_column(&board, 0));
        assert_eq!(false, is_unique_column(&board, 1));
    }

    #[test]
    fn empty_column_is_unique() {
        let board = Board::from_str(
            "
            _ _
            O O",
        ).unwrap();

        assert_eq!(true, is_unique_column(&board, 0));
        assert_eq!(true, is_unique_column(&board, 1));
    }

    #[test]
    fn calc_row_siganture_of_empty_row() {
        let board = Board::from_str(
            "
            _ _ _ _
            O X _ _
            O O O O
            O X O X",
        ).unwrap();

        assert_eq!(None, calc_row_siganture(&board, 0));
        assert_eq!(None, calc_row_siganture(&board, 1));
    }

    #[test]
    fn calc_row_siganture_of_non_empty_row() {
        let board = Board::from_str(
            "
            O O X X
            O X O X
            O O O X
            X X X X",
        ).unwrap();

        assert_eq!(Some(12), calc_row_siganture(&board, 0));
        assert_eq!(Some(10), calc_row_siganture(&board, 1));
        assert_eq!(Some(8), calc_row_siganture(&board, 2));
        assert_eq!(Some(15), calc_row_siganture(&board, 3));
    }
    #[test]
    fn unique_row() {
        let board = Board::from_str(
            "
            X O
            O X",
        ).unwrap();

        assert_eq!(true, is_unique_row(&board, 0));
        assert_eq!(true, is_unique_row(&board, 1));
    }

    #[test]
    fn valid_move() {
        let board = Board::from_str(
            "
            O X X O
            X O O X
            X X O O
            O O X X",
        ).unwrap();

        for x in 0..4 {
            for y in 0..4 {
                assert_eq!(true, is_move_valid(&board, x, y));
            }
        }
    }

    #[test]
    fn regression_board_01() {
        let board = Board::from_str(
            "
            X O X O O X
            O X O X X O
            X O X O O X
            O X O X X O
            X X O X O O
            O O X O X X",
        ).unwrap();

        assert_eq!(false, is_unique_column(&board, 2));
        assert_eq!(false, is_unique_column(&board, 5));
    }

    #[test]
    fn regression_board_02() {
        let board = Board::from_str(
            "
            X O X O O X
            O X O X X O
            X O X O O X
            O X O X X O
            X X O X O O
            O O X O X X",
        ).unwrap();

        assert_eq!(false, is_unique_row(&board, 0));
        assert_eq!(false, is_unique_row(&board, 2));

        assert_eq!(false, is_unique_row(&board, 1));
        assert_eq!(false, is_unique_row(&board, 3));
    }

    #[test]
    fn is_board_valid_for_valid_board() {
        let ok = Board::from_str(
            "
            X O X O
            O X O X
            X X O O
            O O X X",
        ).unwrap();

        assert_eq!(true, is_board_valid(&ok));
    }

    #[test]
    fn is_board_invalid_for_invalid_board() {
        let wrong = Board::from_str(
            "
            O X O X
            X O X O
            X O O O
            O X X O",
        ).unwrap();

        assert_eq!(false, is_board_valid(&wrong));
    }

    #[test]
    fn non_unique_rows() {
        let wrong = Board::from_str(
            "
            X O
            X O",
        ).unwrap();

        assert_eq!(false, are_rows_unique(&wrong));
    }

    #[test]
    fn non_unique_columns() {
        let wrong = Board::from_str(
            "
            X X
            O O",
        ).unwrap();

        assert_eq!(false, are_columns_unique(&wrong));
    }

    #[test]
    fn unbalanced_rows() {
        let wrong = Board::from_str(
            "
            O X O X
            X O X O
            X O O O
            O X X X",
        ).unwrap();

        assert_eq!(false, are_rows_balanced(&wrong));
    }

    #[test]
    fn unbalanced_columns() {
        let wrong = Board::from_str(
            "
            O X O X
            X O X O
            X X O O
            O O X O",
        ).unwrap();

        assert_eq!(false, are_columns_balanced(&wrong));
    }

    #[test]
    fn is_board_valid_for_incomplete_board() {
        let incomplete = Board::from_str(
            "
            _ O X O
            O X O X
            X X O O
            O O X X",
        ).unwrap();

        assert_eq!(false, is_board_valid(&incomplete));
    }

    #[test]
    fn more_then_two_o_in_a_row() {
        let wrong = Board::from_str(
            "
            O X O X
            X O X O
            X O O O
            O X X O",
        ).unwrap();

        assert_eq!(false, has_no_more_than_two_same_neightbors(&wrong));
    }

    #[test]
    fn is_board_full_with_full_board() {
        let full_board = Board::from_str(
            "
            O X O X
            X O X O
            X O O O
            O X X O",
        ).unwrap();
        assert_eq!(true, is_board_full(&full_board));
    }

    #[test]
    fn is_board_full_with_none_full_board() {
        let none_full_board = Board::from_str(
            "
            O X O X
            X _ X O
            X O O O
            O X X O",
        ).unwrap();
        assert_eq!(false, is_board_full(&none_full_board));
    }
}
