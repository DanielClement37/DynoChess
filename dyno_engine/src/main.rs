#![allow(dead_code)]
#![allow(unused_variables)]

mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;
mod search;

use crate::misc::cmdline::{cmd_game_loop};

fn main() {
    //random_ai_loop();
    cmd_game_loop();
}
