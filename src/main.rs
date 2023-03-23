#![allow(dead_code)]
#![allow(unused_variables)]

mod defs;
mod board;
mod misc;
mod movegen;
mod extra;
mod perft;

use std::sync::{Arc, Mutex};
use crate::misc::print;
use crate::movegen::defs::{MoveList, MoveType};
use crate::movegen::MoveGen;
use crate::perft::perft;

fn main() {
    let mut board = board::Board::new();
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".trim();
    board.fen_read(Some(fen)).ok();

    perft::run(Arc::new(Mutex::new(board)), 7, Arc::new(MoveGen::new()))

}