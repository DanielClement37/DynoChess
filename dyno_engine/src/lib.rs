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
use crate::movegen::defs::{Move, MoveType};
use crate::movegen::MoveGen;

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", content = "board")]
enum EngineResponse {
    RegularMove(BoardView),
    Checkmate(BoardView),
    PlayerCheckmate(BoardView),
    InvalidMove(BoardView),
    EnginePanicked,
}

#[wasm_bindgen]
pub fn init_board(flip: bool) -> JsValue {
    let mut board = Board::new();
    board.fen_read(None).ok();
    let board_view = board.to_board_view(flip);
    JsValue::from_serde(&board_view).unwrap()
}

#[wasm_bindgen]
pub fn make_move_js(board_view_js: &JsValue, from: usize, to: usize , isFlipped: bool) -> JsValue {
    let board_view: BoardView = board_view_js.into_serde().unwrap();
    let mut board = Board::from_board_view(board_view);

    let mg = MoveGen::new();
    let legal_moves = mg.generate_legal_moves(&board, MoveType::All);

    let mut chosen_move: Option<Move> = None;
    for i in 0..legal_moves.len() {
        let mv = legal_moves.get_move(i);
        let mv_from = mv.from();
        let mv_to = mv.to();
        if mv_from == from && mv_to == to {
            chosen_move = Some(mv);
            break;
        }
    }

    if chosen_move.is_none() {
        let new_view = board.to_board_view(isFlipped);
        let resp = EngineResponse::InvalidMove(new_view);
        return JsValue::from_serde(&resp).unwrap();
    }

    let mv_to_apply = chosen_move.unwrap();
    let move_worked = board.make(mv_to_apply, &mg);

    if !move_worked {
        let new_view = board.to_board_view(isFlipped);
        let resp = EngineResponse::InvalidMove(new_view);
        return JsValue::from_serde(&resp).unwrap();
    }

    let next_moves = mg.generate_legal_moves(&board, MoveType::All);
    if next_moves.len() == 0 {
        let new_view = board.to_board_view(isFlipped);
        let resp = EngineResponse::Checkmate(new_view);
        return JsValue::from_serde(&resp).unwrap();
    }

    // 5) Otherwise, it's a valid normal move
    let new_view = board.to_board_view(isFlipped);
    let resp = EngineResponse::RegularMove(new_view);
    JsValue::from_serde(&resp).unwrap()
}