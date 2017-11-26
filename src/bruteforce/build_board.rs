use field::{Board};
use bruteforce::possible_move::{calc_possible_moves};
use bruteforce::choose_move::{Move, select_next_move};

pub fn create_board(size:usize) -> Board {
    let mut board = Board::new(size);

    for _ in 0..(size*size) {
        let next_move = select_next_move(&calc_possible_moves(&mut board));
        if let Some(Move { field, x, y }) = next_move {
            board.set(x, y, field);
        }
    }

    board
}