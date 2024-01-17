mod board;
mod defs;
mod eval;
mod extra;
mod misc;
mod movegen;
mod perft;
mod search;

extern crate web_sys;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use gloo_utils::format::JsValueSerdeExt;
use board::Board;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use crate::defs::Sides;

use crate::movegen::defs::{Move, MoveList, MoveType};
use crate::search::alpha_beta;
use crate::search::defs::INF;
use std::panic;


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

///return a serialized version of the board after the given move has been made
/// if the move is not valid return the same board position
#[wasm_bindgen]
pub async fn make_move(board: JsValue, client_mv: usize)-> Result<JsValue, JsValue>{
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    let mut board = serde_wasm_bindgen::from_value(board).unwrap_or_else(|_| panic!("Failed to deserialize board"));
    let mg = movegen::MoveGen::new();
    let mut legal_moves = MoveList::new();
    legal_moves = mg.generate_legal_moves(&board, MoveType::All);

    //try to convert the client move to a move object
    let mv = Move::new(client_mv);

    // Validate the move
    let mut legal = false;
    let mut move_index: u8 = 0;
    for i in 0..legal_moves.len() {
        // Get the move to be executed and counted.
        let m = legal_moves.get_move(i);
        if mv == m {
            legal = true;
            move_index = i;
        }
    }
    if !legal {
        // Return the original board with a custom status indicating an invalid move
        return Ok(JsValue::from_serde(&EngineResponse::InvalidMove).unwrap_or_else(|_| panic!("Failed to serialize invalid move response")));
    }

    // Make the move
    board.make(legal_moves.get_move(move_index), &mg);

    // Check for checkmate
    let mut opp_moves = MoveList::new();
    mg.generate_pseudo_moves(&board, &mut opp_moves, MoveType::All);

    if opp_moves.len() == 0 {
        // Return the board with a custom status indicating checkmate
        return Ok(EngineResponse::Checkmate(board).serialize(&serializer).unwrap_or_else(|_| panic!("Failed to serialize checkmate response")));
    }

    return Ok(EngineResponse::RegularMove(board).serialize(&serializer).unwrap_or_else(|_| panic!("Failed to serialize regular move response")));
}

/// return a serialized version of the board after the engine has made a move
#[wasm_bindgen]
pub async fn make_engine_move(board: JsValue, depth: u8) ->  Result<JsValue, JsValue> {
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    let mut board:Board = serde_wasm_bindgen::from_value(board).unwrap_or_else(|_| panic!("Failed to deserialize board"));
    let mg = movegen::MoveGen::new();

    // Get best AI move from the minimax function here
    let maximizing = board.game_state.active_color == Sides::WHITE as u8;
    let best_move = alpha_beta(&mut board, &mg, depth, -INF, INF, maximizing).1;

    board.make(best_move.unwrap(), &mg); // Make the best move

    let mut player_moves = MoveList::new();
    mg.generate_pseudo_moves(&board, &mut player_moves, MoveType::All);
    if player_moves.len() == 0 {
        // Return the board with a custom status indicating player checkmate
        return Ok(EngineResponse::PlayerCheckmate(board).serialize(&serializer).unwrap_or_else(|_| panic!("Failed to serialize player checkmate response")));
    }

    // Return the updated board with a custom status indicating a regular move
    Ok(EngineResponse::RegularMove(board).serialize(&serializer).unwrap_or_else(|_| panic!("Failed to serialize regular move response")))
}

#[derive(Serialize, Deserialize)]
enum EngineResponse {
    RegularMove(Board),
    Checkmate(Board),
    PlayerCheckmate(Board),
    InvalidMove,
    EnginePanicked,
}