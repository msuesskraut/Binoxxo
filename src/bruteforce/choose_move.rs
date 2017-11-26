use field::Field;
use bruteforce::possible_move::PossibleMove;

use rand::{Rng, thread_rng};

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
}
