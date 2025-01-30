mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;
mod search;
mod frontend;

extern crate web_sys;
use gloo_utils::format::JsValueSerdeExt;
use board::Board;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::frontend::BoardView;

#[wasm_bindgen]
pub fn init_board() -> JsValue {
    let mut board = Board::new();
    board.fen_read(None).ok();
    let frontend_board = board.to_board_view();
    JsValue::from_serde(&frontend_board).unwrap()
}

#[wasm_bindgen]
pub fn make_move_js(board_view_js: &JsValue, from: usize, to: usize) -> JsValue {
    // 1) Deserialize the JS BoardView into a Rust BoardView
    let board_view: BoardView = board_view_js.into_serde().unwrap();

    // 2) Convert BoardView -> Board
    let mut board = Board::from_board_view(board_view); // you'll implement `from_board_view` if needed

    // TODO: 3) Generate, validate, apply the move, etc.
    // (or call your existing `board.make(...)` in combination with your MoveGen, etc.)

    // 4) Return new BoardView or some EngineResponse
    let result = EngineResponse::RegularMove(board);  // or check for checkmate, invalid, etc.

    // 5) Serialize back to JS
    JsValue::from_serde(&result).unwrap()
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", content = "board")]
enum EngineResponse {
    RegularMove(Board),
    Checkmate(Board),
    PlayerCheckmate(Board),
    InvalidMove,
    EnginePanicked,
}