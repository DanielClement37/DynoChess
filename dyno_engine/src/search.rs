pub mod defs;
mod movesorting;

use crate::board::Board;
use crate::eval::evaluate_position;
use crate::movegen::defs::{Move, MoveList, MoveType};
use crate::movegen::MoveGen;
use crate::search::defs::INF;
use crate::search::movesorting::score_moves;

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
    end_time: f64,
) -> (i16, Option<Move>) {

    if depth == 0 || js_sys::Date::now() > end_time{
        return (evaluate_position(board), None);
    }
    if maximizing {
        let mut best_value = -INF;
        let mut best_move: Option<Move> = None;
        let mut value;
        // Generate moves and order them using move ordering function
        let mut move_list = MoveList::new();
        mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);
        move_list = *score_moves(&mut move_list);

        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = alpha_beta(board, mg, depth - 1, alpha, beta, false, end_time).0;
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
        move_list = *score_moves(&mut move_list);

        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            if board.make(m, mg) {
                value = alpha_beta(board, mg, depth - 1, alpha, beta, true, end_time).0;
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

// iterative_deepening_search
pub fn iterative_deepening_search(
    board: &mut Board,
    mg: &MoveGen,
    end_time: f64,
    engine_color: u8,
) -> Option<Move> {
    let mut best_move: Option<Move> = None;

    let mut depth = 1;
    while js_sys::Date::now() < end_time {
        let maximizing = board.game_state.active_color == engine_color;
        if let Some(mv) = alpha_beta(board, mg, depth, -INF, INF, maximizing, end_time).1 {
            best_move = Some(mv);
            web_sys::console::log_1(&depth.into());
        } else {
            break; // Break if no move found for the current depth
        }
        depth += 1;
    }

    best_move
}