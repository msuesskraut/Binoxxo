extern crate binoxxo;

use binoxxo::field::Field;

fn main() {
    let size:usize = 10;
    let board = binoxxo::bruteforce::build_board::create_board(size);

    for y in 0..size {
        let mut line = String::new();
        for x in 0..size {
            match board.get(x, y) {
                Field::X => line += "X ",
                Field::O => line += "O ",
                Field::Empty => line += "_ ",
            }
        }
        println!("{}", line);
    }
}