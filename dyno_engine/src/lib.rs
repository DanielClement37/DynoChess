mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;
mod search;

extern crate web_sys;

use board::Board;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;

use crate::movegen::defs::{MoveList, MoveType};


/// Serializes the initial chess board position and returns it as a JsValue.
#[wasm_bindgen]
pub fn make_initial_position() -> JsValue {
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    let mut board = board::Board::new();
    board.fen_read(None).ok();
   return board.serialize(&serializer).unwrap();
}

/// return a serialized list of legal moves for the given chess board position
#[wasm_bindgen]
pub fn get_legal_moves(val: JsValue) -> JsValue {
    let board = serde_wasm_bindgen::from_value(val).unwrap();
    let mg = movegen::MoveGen::new();
    let mut legal_moves = MoveList::new();
    legal_moves = mg.generate_legal_moves(&board, MoveType::All);
    return serde_wasm_bindgen::to_value(&legal_moves).unwrap();
}
