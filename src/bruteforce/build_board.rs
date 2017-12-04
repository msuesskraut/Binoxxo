use field::{Board, Field};
use bruteforce::possible_move::{PossibleMove, calc_possible_moves};

use rand::{Rng, thread_rng};

fn is_board_full(board : &Board) -> bool {
    let size = board.get_size();
    for x in 0..size {
        for y in 0..size {
            if Field::Empty == board.get(x, y) {
                return false;
            }
        }
    }

    true
}

fn create_board_recursive(board : &mut Board) -> bool {
    if is_board_full(board) {
        return true;
    }

    let moves = calc_possible_moves(board);
    if moves.contains(&PossibleMove::NoMove) {
        return false;
    }

    let mut rng = thread_rng();
    let mut one_moves = moves.iter().filter(|m| match *m {
        &PossibleMove::OneMove(_, _, _) => true,
        _ => false,
    }).map(|m| match *m {
        PossibleMove::OneMove(x, y, field) => (x, y, field),
        _ => unreachable!(),
    }).collect::<Vec<(usize, usize, Field)>>();
    rng.shuffle(&mut one_moves);
    for (x, y, field) in one_moves {
        board.set(x, y, field);
        if create_board_recursive(board) {
            return true;
        }
        board.clear(x, y);
    }

    let mut two_moves = moves.iter().filter(|m| match *m {
        &PossibleMove::TwoMoves(_, _) => true,
        _ => false,
    }).map(|m| match *m {
        PossibleMove::TwoMoves(x, y) => (x, y),
        _ => unreachable!(),
    }).collect::<Vec<(usize, usize)>>();
    rng.shuffle(&mut two_moves);
    for (x, y) in two_moves {
        let mut field_order = vec![Field::X, Field::O];
        rng.shuffle(&mut field_order);
        for field in field_order {
            board.set(x, y, field);
            if create_board_recursive(board) {
                return true;
            }
        }
        board.clear(x, y);
    }

    false
}

pub fn create_board(size:usize) -> Board {
    let mut board = Board::new(size);

    let ok = create_board_recursive(&mut board);

    if !ok {
        panic!("No board found for size {}", size);
    }

    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_full_board() {
        let board = board!(2,
            X O
            O X
        );

        assert_eq!(true, is_board_full(&board));
    }

    #[test]
    fn detect_board_with_empty_fields() {
        let board = board!(2,
            X E
            O X
        );

        assert_eq!(false, is_board_full(&board));
    }
}