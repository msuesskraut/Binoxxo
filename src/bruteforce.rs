use field::{Field, Board};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PossibleMove {
    NoMove,
    OneMove(Field),
    TwoMoves,
}

pub fn calc_possible_move(board: &Board, x : usize, y : usize) -> PossibleMove {
    if Field::Empty == board.get(x, y) {
        PossibleMove::TwoMoves
    }
    else {
        PossibleMove::NoMove
    }
}

pub fn is_valid_move(board: &Board, x : usize, y : usize) -> bool {
    let current = board.get(x, y);

    if y > 1 && current == board.get(x, y - 2) && current == board.get(x, y - 1) {
        false
    }
    else {
        true
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

        assert_eq!(true, is_valid_move(&board, 0, 2));
    }

    #[test]
    fn x_with_adjacent_xx_is_invalid() {
        let board = board!(4,
            X E E E
            X E E E
            X E E E
            E E E E
        );

        assert_eq!(false, is_valid_move(&board, 0, 2));
    }
}
