mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;
mod search;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;


// make a test function that makes initial chess position to convert to wasm
#[wasm_bindgen]
pub fn make_initial_position() -> JsValue {
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    let mut board = board::Board::new();
    board.fen_read(None).ok();
   return board.serialize(&serializer).unwrap();
}