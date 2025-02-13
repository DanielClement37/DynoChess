mod board;
mod defs;
mod eval;
mod extra;
mod frontend;
mod misc;
mod movegen;
mod perft;
mod search;

extern crate web_sys;
use crate::frontend::{BoardView, MoveView};
use crate::movegen::defs::{Move, MoveType};
use crate::movegen::MoveGen;
use board::Board;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
pub fn make_move_js(board_view_js: &JsValue, from: usize, to: usize) -> JsValue {
    let old_view: BoardView = board_view_js.into_serde().unwrap();
    let mut board = Board::from_board_view(old_view.clone());

    // If was_flipped is true, invert user’s from/to
    let engine_from = if old_view.was_flipped {
        flip_square(from)
    } else {
        from
    };
    let engine_to = if old_view.was_flipped {
        flip_square(to)
    } else {
        to
    };

    let mg = MoveGen::new();
    let legal_moves = mg.generate_legal_moves(&board, MoveType::All);

    // Now compare against engine_from / engine_to
    let mut chosen_move: Option<Move> = None;
    for i in 0..legal_moves.len() {
        let mv = legal_moves.get_move(i);
        if mv.from() == engine_from && mv.to() == engine_to {
            chosen_move = Some(mv);
            break;
        }
    }

    if chosen_move.is_none() {
        let new_view = board.to_board_view(old_view.was_flipped);
        return JsValue::from_serde(&EngineResponse::InvalidMove(new_view)).unwrap();
    }

    let mv_to_apply = chosen_move.unwrap();
    let move_worked = board.make(mv_to_apply, &mg);

    if !move_worked {
        let new_view = board.to_board_view(old_view.was_flipped);
        let resp = EngineResponse::InvalidMove(new_view);
        return JsValue::from_serde(&resp).unwrap();
    }

    let next_moves = mg.generate_legal_moves(&board, MoveType::All);
    if next_moves.len() == 0 {
        let new_view = board.to_board_view(old_view.was_flipped);
        let resp = EngineResponse::Checkmate(new_view);
        return JsValue::from_serde(&resp).unwrap();
    }

    let new_view = board.to_board_view(old_view.was_flipped);
    let resp = EngineResponse::RegularMove(new_view);
    JsValue::from_serde(&resp).unwrap()
}

#[wasm_bindgen]
pub fn get_legal_moves_js(board_view_js: &JsValue) -> JsValue {
    let old_view: BoardView = board_view_js.into_serde().unwrap();
    let mut board = Board::from_board_view(old_view.clone());

    let mg = MoveGen::new();
    let legal_moves = mg.generate_legal_moves(&board, MoveType::All);

    let mut moves_vec: Vec<MoveView> = Vec::new();

    for i in 0..legal_moves.len() {
        let mv = legal_moves.get_move(i);
        let (engine_from, engine_to) = (mv.from(), mv.to());

        // Convert engine_from -> visual_from if needed
        let visual_from = if old_view.was_flipped {
            flip_square(engine_from)
        } else {
            engine_from
        };

        let visual_to = if old_view.was_flipped {
            flip_square(engine_to)
        } else {
            engine_to
        };

        moves_vec.push(MoveView {
            from: visual_from as u8,
            to: visual_to as u8,
            piece_type: mv.piece() as u8,
            capture: mv.captured() as u8,
            promotion: mv.promoted() as u8,
        });
    }

    JsValue::from_serde(&moves_vec).unwrap()
}

#[wasm_bindgen]
pub fn flip_board_js(board_view_js: &JsValue) -> JsValue {
    let old_view: BoardView = board_view_js.into_serde().unwrap();
    let toggled = !old_view.was_flipped;
    let mut board = Board::from_board_view(old_view.clone());
    let new_view = board.to_board_view(toggled);
    JsValue::from_serde(&new_view).unwrap()
}

fn flip_square(sq: usize) -> usize {
    let sq_u8 = sq as u8;
    let row = sq_u8 / 8;
    let col = sq_u8 % 8;
    (7 - row) as usize * 8 + col as usize
}
