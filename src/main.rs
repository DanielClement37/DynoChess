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

use crate::misc::cmdline::{ random_ai_loop};

fn main() {
    random_ai_loop();
}
