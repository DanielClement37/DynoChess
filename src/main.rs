#![allow(dead_code)]
#![allow(unused_variables)]

mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;

use crate::misc::print;
use crate::movegen::defs::{MoveList, MoveType};
use crate::movegen::MoveGen;
use crate::perft::perft;
use crate::eval::evaluate_position;
use std::sync::{Arc, Mutex};
use crate::misc::print::position;

fn main() {
    let mut board = board::Board::new();
    board.fen_read(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".trim())).ok();
    position(&board, None);

    //test eval
    let value = evaluate_position(&board);
    println!("eval: {}", value);
}
