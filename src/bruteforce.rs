use field::{Field, Board};

use rand::{Rng, thread_rng};

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
    Some(sig)
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
                .filter(|col| *col != y)
                .map(|col| calc_row_siganture(board, col))
                .filter(|sig| *sig != Some(reference))
                .count() > 0
        }
        None => true,
    }
}

fn is_move_valid(board: &Board, x: usize, y: usize) -> bool {
    is_valid_pair_rule(board, x, y) && is_valid_colum(board, x, y) &&
    is_valid_row(board, x, y) && is_unique_row(board, y) && is_unique_column(board, x)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PossibleMove {
    NoMove,
    OneMove(usize, usize, Field),
    TwoMoves(usize, usize),
}

fn calc_possible_move(board: &mut Board, x: usize, y: usize) -> PossibleMove {
    if Field::Empty == board.get(x, y) {
        board.set(x, y, Field::X);
        let x_possible = is_move_valid(board, x, y);
        board.clear(x, y);
        board.set(x, y, Field::O);
        let y_possible = is_move_valid(board, x, y);
        board.clear(x, y);
        if x_possible && y_possible {
            PossibleMove::TwoMoves(x, y)
        } else if x_possible {
            PossibleMove::OneMove(x, y, Field::X)
        } else if y_possible {
            PossibleMove::OneMove(x, y, Field::O)
        } else {
            PossibleMove::NoMove
        }
    } else {
        PossibleMove::NoMove
    }
}

pub fn calc_possible_moves(board: &mut Board) -> Vec<PossibleMove> {
    let mut result = Vec::new();
    for x in 0..board.get_size() {
        for y in 0..board.get_size() {
            if Field::Empty == board.get(x, y) {
                result.push(calc_possible_move(board, x, y))
            }
        }
    }
    result
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    field: Field,
    x: usize,
    y: usize,
}

pub fn select_next_move(possible_moves: &[PossibleMove]) -> Option<Move> {
    if possible_moves.is_empty() || possible_moves.contains(&PossibleMove::NoMove) {
        None
    } else {
        let single_options = possible_moves
            .iter()
            .filter(|e| match *(*e) {
                        PossibleMove::OneMove(_, _, _) => true,
                        _ => false,
                    })
            .collect::<Vec<&PossibleMove>>();
        let mut rng = thread_rng();

        if !single_options.is_empty() {
            match rng.choose(&single_options) {
                Some(&&PossibleMove::OneMove(x, y, field)) => Some(Move { field, x, y }),
                _ => unreachable!(),
            }
        } else {
            match rng.choose(possible_moves) {
                Some(&PossibleMove::TwoMoves(x, y)) => {
                    let field = if rng.gen() { Field::X } else { Field::O };
                    Some(Move { field, x, y })
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_none_if_one_impossible_move_in_list() {
        assert_eq!(None,
                   select_next_move(&[PossibleMove::OneMove(1, 1, Field::X),
                                      PossibleMove::TwoMoves(2, 2),
                                      PossibleMove::NoMove]));
    }

    #[test]
    fn select_random_one_move_if_list_of_possible_moves_contains_one_move() {
        let possible_moves = vec![PossibleMove::OneMove(1, 1, Field::X),
                                  PossibleMove::OneMove(2, 1, Field::O),
                                  PossibleMove::TwoMoves(2, 2)];
        let next_move = select_next_move(&possible_moves).unwrap();
        assert!(possible_moves.contains(&PossibleMove::OneMove(next_move.x,
                                                               next_move.y,
                                                               next_move.field)));
    }

    #[test]
    fn select_random_two_move_if_list_of_possible_moves_contains_only_two_moves() {
        let possible_moves = vec![PossibleMove::TwoMoves(1, 1),
                                  PossibleMove::TwoMoves(2, 1),
                                  PossibleMove::TwoMoves(2, 2)];
        let next_move = select_next_move(&possible_moves).unwrap();
        assert!(possible_moves.contains(&PossibleMove::TwoMoves(next_move.x, next_move.y)));
    }

    #[test]
    fn get_possible_moves_for_all_empty_fields() {
        let mut board = board!(4,
            X X E E
            O O X X
            O X X O
            E E X O
        );

        let possible_moves = calc_possible_moves(&mut board);
        assert_eq!(4, possible_moves.len());
    }

    #[test]
    fn get_possible_moves_does_not_change_board() {
        let mut board = board!(4,
            X X E E
            O O X X
            O X X O
            E E X O
        );

        let _ = calc_possible_moves(&mut board);
        assert_eq!(board!(4,
            X X E E
            O O X X
            O X X O
            E E X O
        ),
                   board);
    }

    #[test]
    fn get_possible_one_moves_for_all_empty_fields() {
        let mut board = board!(4,
            X X E E
            O O X X
            O X X O
            E E X O
        );

        let possible_moves = calc_possible_moves(&mut board);
        assert!(possible_moves.contains(&PossibleMove::OneMove(2, 0, Field::O)));
        assert!(possible_moves.contains(&PossibleMove::OneMove(1, 3, Field::O)));
        assert!(possible_moves.contains(&PossibleMove::OneMove(0, 3, Field::X)));
        assert!(possible_moves.contains(&PossibleMove::NoMove));
    }

    #[test]
    fn next_to_a_single_x_are_two_moves_possible() {
        let mut board = board!(4,
            X E E E
            E E E E
            E E E E
            E E E E
        );

        let possible_moves = calc_possible_moves(&mut board);

        assert!(possible_moves.contains(&PossibleMove::TwoMoves(1, 0)));
    }

    #[test]
    fn move_not_possible_for_set_field() {
        let mut board = board!(2,
            X E
            E E
        );

        assert_eq!(PossibleMove::NoMove, calc_possible_move(&mut board, 0, 0));
    }

    #[test]
    fn two_options_for_empty_board() {
        let mut board = board!(2,
            E E
            E E
        );

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(PossibleMove::TwoMoves(x, y),
                           calc_possible_move(&mut board, x, y));
            }
        }
    }

    #[test]
    fn only_option_x_possible() {
        let mut board = board!(2,
            O O
            O E
        );

        assert_eq!(PossibleMove::OneMove(1, 1, Field::X),
                   calc_possible_move(&mut board, 1, 1));
    }

    #[test]
    fn only_option_o_possible() {
        let mut board = board!(4,
            E X X O
            X X O O
            O O X X
            X O O X
        );

        assert_eq!(PossibleMove::OneMove(0, 0, Field::O),
                   calc_possible_move(&mut board, 0, 0));
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

    #[test]
    fn calc_row_siganture_of_empty_row() {
        let board = board!(4,
            E E E E
            O X E E
            O O O O
            O X O X
        );

        assert_eq!(None, calc_row_siganture(&board, 0));
        assert_eq!(None, calc_row_siganture(&board, 1));
    }

    #[test]
    fn calc_row_siganture_of_non_empty_row() {
        let board = board!(4,
            O O X X
            O X O X
            O O O X
            X X X X
        );

        assert_eq!(Some(12), calc_row_siganture(&board, 0));
        assert_eq!(Some(10), calc_row_siganture(&board, 1));
        assert_eq!(Some(8), calc_row_siganture(&board, 2));
        assert_eq!(Some(15), calc_row_siganture(&board, 3));
    }
    #[test]
    fn unique_row() {
        let board = board!(2,
            X O
            O X
        );

        assert_eq!(true, is_unique_row(&board, 0));
        assert_eq!(true, is_unique_row(&board, 1));
    }

    #[test]
    fn valid_move() {
        let board = board!(4,
            O X X O
            X O O X
            X X O O
            O O X X
        );

        for x in 0..4 {
            for y in 0..4 {
                assert_eq!(true, is_move_valid(&board, x, y));
            }
        }
    }
}
