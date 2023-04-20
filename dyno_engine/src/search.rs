pub mod defs;

use crate::board::Board;
use crate::eval::evaluate_position;
use crate::movegen::defs::{Move, MoveList, MoveType};
use crate::movegen::MoveGen;
use crate::search::defs::INF;

pub fn minimax(board: &mut Board, mg: &MoveGen, depth: u8, maximizing: bool) -> ( i16, Option<Move>) {
    if depth == 0 {
        return (evaluate_position(board), None);
    }
    if maximizing {
        let mut best_value = -INF;
        let mut best_move: Option<Move> = None;
        let mut value;
        //generate moves
        let mut move_list = MoveList::new();
        mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = minimax(board, mg, depth - 1, false).0;
                board.unmake();
            } else {
                continue;
            }
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
        }

        (best_value, best_move)
    } else {
        let mut best_value = INF;
        let mut best_move: Option<Move> = None;
        let mut value;
        //generate moves
        let mut move_list = MoveList::new();
        mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = minimax(board, mg, depth - 1, true).0;
                board.unmake();
            } else {
                continue;
            }
            if value < best_value {
                best_value = value;
                best_move = Some(m);
            }
        }

        (best_value, best_move)
    }
}

//alpha beta pruning minimax function with move ordering
pub fn alpha_beta(
    board: &mut Board,
    mg: &MoveGen,
    depth: u8,
    mut alpha: i16,
    mut beta: i16,
    maximizing: bool,
) -> (i16, Option<Move>) {
    if depth == 0 {
        return (evaluate_position(board), None);
    }
    if maximizing {
        let mut best_value = -INF;
        let mut best_move: Option<Move> = None;
        let mut value;
        // Generate moves and order them using move ordering function
        let mut move_list = MoveList::new();
        mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = alpha_beta(board, mg, depth - 1, alpha, beta, false).0;
                board.unmake();
            } else {
                continue;
            }
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
            if best_value >= beta {
                break;
            }
            alpha = std::cmp::max(alpha, best_value);
        }
        (best_value, best_move)
    } else {
        let mut best_value = INF;
        let mut best_move: Option<Move> = None;
        let mut value;
        // Generate moves and order them using move ordering function
        let mut move_list = MoveList::new();
        mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);
        //order_moves(board, &mut move_list, alpha, beta); // Move ordering
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = alpha_beta(board, mg, depth - 1, alpha, beta, true).0;
                board.unmake();
            } else {
                continue;
            }
            if value < best_value {
                best_value = value;
                best_move = Some(m);
            }
            if best_value <= alpha {
                break;
            }
            beta = std::cmp::min(beta, best_value);
        }
        (best_value, best_move)
    }
}