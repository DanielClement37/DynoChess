#![allow(dead_code)]
#![allow(unused_variables)]

mod defs;
mod board;
mod misc;
mod movegen;
mod extra;

use crate::misc::print;
use crate::movegen::defs::{MoveList, MoveType};

fn main() {
    let mut board = board::Board::new();
    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -".trim();
    board.fen_read(Some(fen)).ok();
    //board.fen_read(None).ok();


    print::position(&board, None);
    let move_gen = movegen::MoveGen::new();
    let mut movelist = MoveList::new();
    move_gen.generate_moves(&board, &mut movelist, MoveType::All);

    let mut legal_moves_list = MoveList::new();

    println!("num pseudo legal moves: {}", movelist.len());

    for i in 0..movelist.len() {
        let m = movelist.get_move(i);
        let is_legal = board.make(m,&move_gen);

        if is_legal {
            //println!("{}", m.as_string());
            legal_moves_list.push(m);
            board.unmake();
        }
        //println!("{}", m.as_string());

    }
   println!("num legal moves: {}", legal_moves_list.len());




}