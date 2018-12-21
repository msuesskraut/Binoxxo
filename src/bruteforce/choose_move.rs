//! This module implements selecting a move from a list of given moves with
//! `fn` [`select_next_move`](fn.select_next_move).
//! The selected move is represented as `struct` [`Move`](struct.Move.html).

use bruteforce::possible_move::PossibleMove;
use field::Field;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Enum whether the selected move was taken because of
/// it was the only possible move for the field or
/// randomly chosen from multiple possible moves
/// (with both X and O possible for the field).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveSelection {
    /// was randomly selected from multiple options (either X or O)
    Random,
    /// was the only possible move available
    Fixed,
}

/// Next move to perform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    /// Either `X` or `O`.
    pub field: Field,
    /// column of next move
    pub x: usize,
    /// row of next move
    pub y: usize,
    /// whether it was the only possible move or
    /// randomly selected from multiple possible moves
    pub was_random: MoveSelection,
}

/// Returns from a list of possible moves `possible_moves` the next move to take.
///
/// If one of the `possible_moves` is impossible, `select_next_move` returns `None`,
/// because from here on valid board can be constructed any more.
///
/// Otherwise it prefers fixed options (fields where only X or O are possible but not both).
/// The return move is marked as [`MoveSelection`](enum.MoveSelection.html)`::Fixed`.
///
/// Only if there are no fixed options, it returns a randomly chosen move,
/// which is marked as [`MoveSelection`](enum.MoveSelection.html)`::Random`.
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
            match single_options.choose(&mut rng) {
                Some(&&PossibleMove::OneMove(x, y, field)) => {
                    let was_random = MoveSelection::Fixed;
                    Some(Move {
                        field,
                        x,
                        y,
                        was_random,
                    })
                }
                _ => unreachable!(),
            }
        } else {
            match possible_moves.choose(&mut rng) {
                Some(&PossibleMove::TwoMoves(x, y)) => {
                    let field = if rng.gen() { Field::X } else { Field::O };
                    let was_random = MoveSelection::Random;
                    Some(Move {
                        field,
                        x,
                        y,
                        was_random,
                    })
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
        assert_eq!(
            None,
            select_next_move(&[
                PossibleMove::OneMove(1, 1, Field::X),
                PossibleMove::TwoMoves(2, 2),
                PossibleMove::NoMove,
            ],)
        );
    }

    #[test]
    fn select_random_one_move_if_list_of_possible_moves_contains_one_move() {
        let possible_moves = vec![
            PossibleMove::OneMove(1, 1, Field::X),
            PossibleMove::OneMove(2, 1, Field::O),
            PossibleMove::TwoMoves(2, 2),
        ];
        let next_move = select_next_move(&possible_moves).unwrap();
        assert!(possible_moves.contains(&PossibleMove::OneMove(
            next_move.x,
            next_move.y,
            next_move.field,
        )));
    }

    #[test]
    fn select_random_two_move_if_list_of_possible_moves_contains_only_two_moves() {
        let possible_moves = vec![
            PossibleMove::TwoMoves(1, 1),
            PossibleMove::TwoMoves(2, 1),
            PossibleMove::TwoMoves(2, 2),
        ];
        let next_move = select_next_move(&possible_moves).unwrap();
        assert!(possible_moves.contains(&PossibleMove::TwoMoves(next_move.x, next_move.y,)));
    }
}
