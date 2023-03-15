mod defs;
mod board;
mod misc;

use crate::misc::print;

fn main() {
    let mut board = board::Board::new();
    board.fen_read(None).ok();
    print::position(&board, None);
}