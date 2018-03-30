//! Implements a recursive brute force puzzle generator.
//! First it generates a random and valid binoxxo board.
//! Then it takes fields away again (by setting them to `Empty`) and
//! returns the resulting incomplete board as puzzle.
//! Because the board was constructed from a valid board, there exists
//! at least one valid solution for the board.

use bruteforce::choose_move::{select_next_move, Move, MoveSelection};
use bruteforce::possible_move::calc_possible_moves;
use field::Board;

use rand::{thread_rng, Rng};

struct Game {
    board: Board,
    moves: Vec<Move>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            board: Board::new(size),
            moves: Vec::new(),
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
        } else {
            false
        }
    }

    fn undo_moves(&mut self, number: usize) {
        assert!(number <= self.moves.len());

        for _ in 0..number {
            let m = self.moves.pop().unwrap();
            self.board.clear(m.x, m.y);
        }
    }

    fn build_full_game(size: usize, max_tries: usize) -> Option<Game> {
        let mut game = Game::new(size);
        let mut rng = thread_rng();

        for _ in 0..max_tries {
            if game.is_full() {
                return Some(game);
            }
            if !game.new_move() {
                let max = game.moves.len() - 1;
                let number_of_moves = rng.gen_range(1, max);
                game.undo_moves(number_of_moves);
            }
        }

        if game.is_full() {
            Some(game)
        } else {
            None
        }
    }

    pub fn build_full_board(size: usize, max_tries: usize) -> Option<Board> {
        Some(Game::build_full_game(size, max_tries)?.board)
    }

    pub fn build_puzzle_board(size: usize, max_tries: usize, guesses: usize) -> Option<Board> {
        let game = Game::build_full_game(size, max_tries)?;

        let mut board = game.board;
        let mut moves = game.moves;
        let mut guesses = guesses;

        while let Some(m) = moves.pop() {
            match m.was_random {
                MoveSelection::Fixed => board.clear(m.x, m.y),
                MoveSelection::Random => if 0 == guesses {
                    break;
                } else {
                    guesses -= 1;
                },
            }
        }

        Some(board)
    }
}

/// Returns a valid and full binoxxo board of side length `size`.
///
/// # Panics
///
/// Panics if `size` is odd or zero.
///
/// May also panic if it didn't find a valid board. It just does a limited
/// number of tries in order to avoid to long search of the brute force
/// algorithm. This however leads to the small chance, that the algorithm does
/// not find a valid board in the limited number of tries.
/// No such panic was yet discovered while testing.
pub fn create_full_board(size: usize) -> Board {
    let max_tries = size * size * 100;
    if let Some(board) = Game::build_full_board(size, max_tries) {
        return board;
    }

    panic!("No board found for size {} after {} tries", size, max_tries);
}

/// Returns a binoxxo puzzle board of side length `size`.
/// There are some empty fields on the board and there exists at-least
/// one valid board, which can be constructed from the puzzle.
///
/// # Panics
///
/// Panics if `size` is odd or zero.
///
/// May also panic if it didn't find a valid board.
/// See `fn` [`create_full_board`](fn.create_full_board.html) for details.
pub fn create_puzzle_board(size: usize, guesses: usize) -> Board {
    let max_tries = size * size * 100;
    if let Some(board) = Game::build_puzzle_board(size, max_tries, guesses) {
        return board;
    }

    panic!(
        "No board found for size {} with {} guesses after {} tries",
        size, guesses, max_tries
    );
}
