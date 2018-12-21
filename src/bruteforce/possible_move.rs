//! This module contains the code to determine the options for any
//! empty field of a board: `fn` [`calc_possible_moves`](fn.calc_possible_moves).

use bruteforce::rules::is_move_valid;
use field::{Board, Field};

/// The options for a given empty field.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PossibleMove {
    /// no valid move possible (neither X nor O)
    NoMove,
    /// exactly one move possible (either X or O)
    /// - 1. tuple entry is the column `x`
    /// - 2. tuple entry is the row `y`
    /// - 3. tuple entry is the field (either `X` or `O`, but not `Empty`)
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

/// For all empty fields on board `board` returns the possible moves.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn get_possible_moves_for_all_empty_fields() {
        let mut board = Board::from_str(
            "
            X X _ _
            O O X X
            O X X O
            _ _ X O",
        )
        .unwrap();

        let possible_moves = calc_possible_moves(&mut board);
        assert_eq!(4, possible_moves.len());
    }

    #[test]
    fn get_possible_moves_does_not_change_board() {
        let mut board = Board::from_str(
            "
            X X _ _
            O O X X
            O X X O
            _ _ X O",
        )
        .unwrap();

        let _ = calc_possible_moves(&mut board);
        assert_eq!(
            Board::from_str(
                "
            X X _ _
            O O X X
            O X X O
            _ _ X O"
            )
            .unwrap(),
            board
        );
    }

    #[test]
    fn get_possible_one_moves_for_all_empty_fields() {
        let mut board = Board::from_str(
            "
            X X _ _
            O O X X
            O X X O
            _ _ X O",
        )
        .unwrap();

        let possible_moves = calc_possible_moves(&mut board);
        assert!(possible_moves.contains(&PossibleMove::OneMove(2, 0, Field::O),));
        assert!(possible_moves.contains(&PossibleMove::OneMove(1, 3, Field::O),));
        assert!(possible_moves.contains(&PossibleMove::OneMove(0, 3, Field::X),));
        assert!(possible_moves.contains(&PossibleMove::NoMove));
    }

    #[test]
    fn next_to_a_single_x_are_two_moves_possible() {
        let mut board = Board::from_str(
            "
            X _ _ _
            _ _ _ _
            _ _ _ _
            _ _ _ _",
        )
        .unwrap();

        let possible_moves = calc_possible_moves(&mut board);

        assert!(possible_moves.contains(&PossibleMove::TwoMoves(1, 0)));
    }

    #[test]
    fn move_not_possible_for_set_field() {
        let mut board = Board::from_str(
            "
            X _
            _ _",
        )
        .unwrap();

        assert_eq!(PossibleMove::NoMove, calc_possible_move(&mut board, 0, 0));
    }

    #[test]
    fn two_options_for_empty_board() {
        let mut board = Board::from_str(
            "
            _ _
            _ _",
        )
        .unwrap();

        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(
                    PossibleMove::TwoMoves(x, y),
                    calc_possible_move(&mut board, x, y)
                );
            }
        }
    }

    #[test]
    fn only_option_x_possible() {
        let mut board = Board::from_str(
            "
            O O
            O _",
        )
        .unwrap();

        assert_eq!(
            PossibleMove::OneMove(1, 1, Field::X),
            calc_possible_move(&mut board, 1, 1)
        );
    }

    #[test]
    fn only_option_o_possible() {
        let mut board = Board::from_str(
            "
            _ X X O
            X X O O
            O O X X
            X O O X",
        )
        .unwrap();

        assert_eq!(
            PossibleMove::OneMove(0, 0, Field::O),
            calc_possible_move(&mut board, 0, 0)
        );
    }
}
