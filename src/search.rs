mod defs;

use crate::board::Board;
use crate::eval::evaluate_position;
use crate::movegen::defs::{Move, MoveType};
use crate::movegen::MoveGen;
use crate::search::defs::INF;

pub fn minimax(board: &mut Board, mg: &MoveGen, depth: u8, maximizing: bool) -> i16 {
    if depth == 0 {
        return evaluate_position(board);
    }
    if maximizing {
        let mut best_value = -INF;
        //generate moves
        let move_list = mg.generate_legal_moves(board, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            board.make(m, mg);
            let value = minimax(board, mg, depth - 1, false);
            board.unmake();
            best_value = best_value.max(value); // Update best_value with maximum value
        }

        best_value

    } else {
        let mut best_value = INF;
        //generate moves
        let move_list = mg.generate_legal_moves(board, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            board.make(m, mg);
            let value = minimax(board, mg, depth - 1, true);
            board.unmake();
            best_value = best_value.min(value); // Update best_value with minimum value
        }

        best_value
    }
}

pub fn get_best_move(board: &mut Board, mg: &MoveGen, depth: u8, maximizing: bool) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_value = if maximizing { i16::MIN } else { i16::MAX };

    let move_list = mg.generate_legal_moves(board, MoveType::All);
    for i in 0..move_list.len() {
        let m = move_list.get_move(i);
        board.make(m, mg);
        let value = minimax(board, mg, depth - 1, !maximizing);
        board.unmake();

        if maximizing {
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
        } else {
            if value < best_value {
                best_value = value;
                best_move = Some(m);
            }
        }
    }

    best_move
}