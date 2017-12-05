use field::Board;
use bruteforce::possible_move::calc_possible_moves;
use bruteforce::choose_move::{Move, select_next_move};

use rand::{Rng, thread_rng};

struct Game {
    board: Board,
    moves: Vec<Move>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            board : Board::new(size),
            moves : Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        let size = self.board.get_size();
        self.moves.len() == (size * size)
    }

    fn new_move(&mut self) -> bool {
        assert!(!self.is_full());

        let possible_moves = calc_possible_moves(&mut self.board);
        if let Some(m) = select_next_move(&possible_moves) {
            self.board.set(m.x, m.y, m.field);
            self.moves.push(m);
            true
        }
        else {
            false
        }
    }

    fn undo_moves(&mut self, number:usize) {
        assert!(number <= self.moves.len());

        for _ in 0..number {
            let m = self.moves.pop().unwrap();
            self.board.clear(m.x, m.y);
        }
    }

    pub fn build_board(size: usize, max_tries: usize) -> Option<Board> {
        let mut game = Game::new(size);
        let mut rng = thread_rng();

        for _ in 0..max_tries {
            if game.is_full() {
                return Some(game.board)
            }
            if !game.new_move() {
                let max = game.moves.len() - 1;
                let number_of_moves = rng.gen_range(1, max);
                game.undo_moves(number_of_moves);                
            }
        }

        if game.is_full() {
            Some(game.board)
        }
        else {
            None
        }
    }
}

pub fn create_board(size:usize) -> Board {
    let max_tries = size * size * 100;
    if let Some(board) = Game::build_board(size, max_tries) {
        return board;
    }

    panic!("No board found for size {} after {} tries", size, max_tries);
}
