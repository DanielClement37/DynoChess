use crate::board::Board;
use crate::eval::evaluate_position;
use crate::misc::print::position;
use crate::movegen::defs::{Move, MoveType};
use crate::movegen::MoveGen;

pub fn minimax(board: &mut Board, mg: &MoveGen, depth: u8, maximizing: bool) -> i16 {
    if depth == 0 {
        return evaluate_position(board);
    }
    if maximizing {
        let mut best_value = -9999;
        let mut best_move:Option<Move> = None;
        //generate moves
        let move_list = mg.generate_legal_moves(board, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            board.make(m, mg);
            let value = minimax(board, mg, depth - 1, false);
            board.unmake();

            if value > best_value {
                best_value = value;
                best_move = Some(m); // Update the best move
            }
        }

        // Return the best move
        if let Some(mv) = best_move {
            board.make(mv, mg); // Make the best move on the actual board
            println!("Best move: {}", mv.as_string());
            position(board, None);
        }

        best_value

    } else {
        let mut best_value = 9999;
        let mut best_move:Option<Move> = None;
        //generate moves
        let move_list = mg.generate_legal_moves(board, MoveType::All);
        for i in 0..move_list.len() {
            // Get the move to be executed and counted.
            let m = move_list.get_move(i);
            board.make(m, mg);
            let value = minimax(board, mg, depth - 1, true);
            board.unmake();

            if value < best_value {
                best_value = value;
                best_move = Some(m); // Update the best move
            }
        }

        // Return the best move
        if let Some(mv) = best_move {
            board.make(mv, mg); // Make the best move on the actual board

            println!("Best move: {}", mv.as_string());
            position(board, None);
        }

        best_value
    }
}