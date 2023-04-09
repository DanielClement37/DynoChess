#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use crate::board::Board;
use crate::board::defs::{SQUARE_NAME};
use crate::defs::{ Sides};
use crate::misc::print::position;
use crate::movegen::defs::{ MoveType};
use crate::movegen::MoveGen;
use crate::search::{get_best_move};

pub fn cmd_game_loop(){
    let mut fen = String::new();
    let mut side_string = String::new();
    let mut depth_string = String::new();

    println!("Enter the FEN string for the starting position or press enter to start from default position");
    std::io::stdin().read_line(&mut fen).expect("Failed to read line");
    println!("Enter the side to play as (white or black)");
    std::io::stdin().read_line(&mut side_string).expect("Failed to read line");
    println!("Enter the depth of search");
    std::io::stdin().read_line(&mut depth_string).expect("Failed to read line");

    //convert the strings to the correct types
    let fen = fen.trim();
    let user_side = match side_string.trim() {
        "white" => {Sides::WHITE}
        "black" => {Sides::BLACK }
        _ => {panic!("Invalid side") }
    };

    let depth = depth_string.trim().parse::<u8>().expect("Failed to parse depth");

    //create a board
    let mut board = Board::new();
    let mg = MoveGen::new();

    if fen != "" {
        board.fen_read(Some(fen)).ok();
    } else {
        board.fen_read(None).ok();
    }

    //print the starting position
    position(&board, None);

    if board.game_state.active_color != user_side as u8 {
        //check for checkmate
        let opp_moves = mg.generate_legal_moves(&board, MoveType::All);

        if opp_moves.len() == 0 {
            println!("Checkmate You Win!");
            return;
        }

        //Get best ai move from the minimax function here
        let maximizing = board.game_state.active_color == Sides::WHITE as u8;
        let best_move = get_best_move(&mut board, &mg, depth,maximizing); // Call the minimax function
        board.make(best_move.unwrap(), &mg); // Make the best move
        position(&board, Some(best_move.unwrap().to() as u8)); // Print the updated position

        let player_moves = mg.generate_legal_moves(&board, MoveType::All);
        if player_moves.len() == 0 {
            println!("Checkmate You Lose!");
            return;
        }
    }

    //start a loop that asks for user input for move
    //if the user enters "quit", exit the loop
    while board.game_state.active_color == user_side as u8 {
        //ask for a move
        let mut move_string = String::new();
        println!("Enter a move");
        std::io::stdin().read_line(&mut move_string).expect("Failed to read line");
        let move_string = move_string.trim();
        if move_string == "quit" {
            break;
        }
        //split string into spaces
        let move_string_parts: Vec<&str> = move_string.split(" ").collect();
        if move_string_parts.len() != 2 {
            println!("Invalid move. Please try again");
            continue;
        }
        //convert the strings to the correct types
        let from = SQUARE_NAME.iter().position(|&r| r == move_string_parts[0]).unwrap();
        let to = SQUARE_NAME.iter().position(|&r| r == move_string_parts[1]).unwrap();

        //generate the legal moves
        let legal_moves = mg.generate_legal_moves(&board, MoveType::All);
        let mut legal = false;
        let mut move_index: u8 = 0;
        for i in 0..legal_moves.len() {
            // Get the move to be executed and counted.
            let m = legal_moves.get_move(i);
            if m.from() == from && m.to() == to {
                legal = true;
                move_index = i;
            }
        }

        if !legal {
            println!("Illegal move. Please try again");
            continue;
        }
        board.make(legal_moves.get_move(move_index), &mg);

        position(&board, Some(legal_moves.get_move(move_index).to() as u8));

        // check for checkmate
        let opp_moves = mg.generate_legal_moves(&board, MoveType::All);
        if opp_moves.len() == 0 {
            println!("Checkmate You Win!");
            return;
        }

        //Get best ai move from the minimax function here
        let maximizing = board.game_state.active_color == Sides::WHITE as u8;
        let best_move = get_best_move(&mut board, &mg, depth,maximizing); // Call the minimax function
        board.make(best_move.unwrap(), &mg); // Make the best move
        position(&board, Some(best_move.unwrap().to() as u8)); // Print the updated position

        let player_moves = mg.generate_legal_moves(&board, MoveType::All);
        if player_moves.len() == 0 {
            println!("Checkmate You Lose!");
            return;
        }
    }
}

pub fn random_ai_loop(){
    let mut fen = String::new();
    let mut side_string = String::new();

    println!("Enter the FEN string for the starting position or press enter to start from default position");
    std::io::stdin().read_line(&mut fen).expect("Failed to read line");
    println!("Enter the side to play as (white or black)");
    std::io::stdin().read_line(&mut side_string).expect("Failed to read line");

    //convert the strings to the correct types
    let fen = fen.trim();
    let user_side = match side_string.trim() {
        "white" => {Sides::WHITE}
        "black" => {Sides::BLACK }
        _ => {panic!("Invalid side") }
    };

    //create a board
    let mut board = Board::new();
    let mg = MoveGen::new();

    if fen != "" {
        board.fen_read(Some(fen)).ok();
    } else {
        board.fen_read(None).ok();
    }

    //print the starting position
    position(&board, None);

    if board.game_state.active_color != user_side as u8 {
        //check for checkmate
        let opp_moves = mg.generate_legal_moves(&board, MoveType::All);

        if opp_moves.len() == 0 {
            println!("Checkmate You Win!");
            return;
        }
        let random_move = opp_moves.get_move(rand::thread_rng().gen_range(0..opp_moves.len()));

        println!("Opponent move: {} {}", SQUARE_NAME[random_move.from() as usize], SQUARE_NAME[random_move.to() as usize]);
        board.make(random_move, &mg);

        position(&board, Some(random_move.to() as u8));

        let player_moves = mg.generate_legal_moves(&board, MoveType::All);
        if player_moves.len() == 0 {
            println!("Checkmate You Lose!");
            return;
        }
    }

    //start a loop that asks for user input for move
    //if the user enters "quit", exit the loop
    while board.game_state.active_color == user_side as u8 {
        //ask for a move
        let mut move_string = String::new();
        println!("Enter a move");
        std::io::stdin().read_line(&mut move_string).expect("Failed to read line");
        let move_string = move_string.trim();
        if move_string == "quit" {
            break;
        }
        //split string into spaces
        let move_string_parts: Vec<&str> = move_string.split(" ").collect();
        if move_string_parts.len() != 2 {
            println!("Invalid move. Please try again");
            continue;
        }
        //convert the strings to the correct types
        let from = SQUARE_NAME.iter().position(|&r| r == move_string_parts[0]).unwrap();
        let to = SQUARE_NAME.iter().position(|&r| r == move_string_parts[1]).unwrap();

        //generate the legal moves
        let legal_moves = mg.generate_legal_moves(&board, MoveType::All);
        let mut legal = false;
        let mut move_index: u8 = 0;
        for i in 0..legal_moves.len() {
            // Get the move to be executed and counted.
            let m = legal_moves.get_move(i);
            if m.from() == from && m.to() == to {
                legal = true;
                move_index = i;
            }
        }

        if !legal {
            println!("Illegal move. Please try again");
            continue;
        }
        board.make(legal_moves.get_move(move_index), &mg);

        position(&board, Some(legal_moves.get_move(move_index).to() as u8));

        //check for checkmate
        let opp_moves = mg.generate_legal_moves(&board, MoveType::All);

        if opp_moves.len() == 0 {
            println!("Checkmate You Win!");
            break;
        }
        let random_move = opp_moves.get_move(rand::thread_rng().gen_range(0..opp_moves.len()));

        println!("Opponent move: {} {}", SQUARE_NAME[random_move.from() as usize], SQUARE_NAME[random_move.to() as usize]);
        board.make(random_move, &mg);

        position(&board, Some(random_move.to() as u8));

        let player_moves = mg.generate_legal_moves(&board, MoveType::All);
        if player_moves.len() == 0 {
            println!("Checkmate You Lose!");
            break;
        }

    }
}