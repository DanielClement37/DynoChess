use crate::{
    board::Board,
    misc::print,
    movegen::{
        defs::{ MoveType},
        MoveGen,
    },
};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use crate::movegen::defs::MoveList;
use web_sys::console;

// This function runs perft(), while collecting speed information.
// It uses iterative deepening, so when running perft(7), it will output
// the results of perft(1) up to and including perft(7).
pub fn run(
    board: Arc<Mutex<Board>>,
    depth: i8,
    mg: Arc<MoveGen>,
) -> u64 {
    let mut total_time: f64 = 0.0;
    let mut total_nodes: u64 = 0;
    let hash_full = String::from("");
    let mut last_leaf_nodes: u64 = 0;

    // Create a mutex guard for the board, so it can be safely cloned.
    // Panic if the guard can't be created, because something is wrong with
    // the main engine thread.
    let mtx_board = board.lock().expect("Lock failed in perft::run()");

    // Clone the locked board for local use.
    let mut local_board = mtx_board.clone();

    // The function now has its own local board. Drop the guard. It is not
    // necessary to keep the lock until perft runs out.
    drop(mtx_board);

    // Use web_sys::console::log_1 instead of println!
    console::log_1(&format!("Benchmarking perft 1-{}:", depth).into());

    // Perform all perfts for depths 1 up to and including "depth"
    for d in 1..=depth {
        // Current time
        let start = js_sys::Date::now();
        let mut leaf_nodes = 0;
        leaf_nodes += perft(&mut local_board, d, &mg);

        // Measure time and speed
        let elapsed = js_sys::Date::now() - start;
        let leaves_per_second = ((leaf_nodes - last_leaf_nodes) as f64 / elapsed * 1000.0).floor();

        // Add tot totals for the final calculation at the very end.
        total_time += elapsed;
        total_nodes += leaf_nodes;

        // Use web_sys::console::log_1 instead of println!
        console::log_1(&format!(
            "Perft {}: {} ({} ms, {} leaves/sec{})",
            d, leaf_nodes, elapsed, leaves_per_second, hash_full
        ).into());
        last_leaf_nodes = leaf_nodes;
    }

    // Final calculation of the entire time taken and the average speed of leaves/second.
    let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
    // Use web_sys::console::log_1 instead of println!
    console::log_1(&format!("Total time spent: {} ms", total_time).into());
    console::log_1(&format!("Execution speed: {} leaves/second", final_lnps).into());

    last_leaf_nodes
}

// This is the actual Perft function. It is public, because it is used by
// the "testsuite" module.
pub fn perft(
    board: &mut Board,
    depth: i8,
    mg: &MoveGen,
) -> u64 {
    let mut leaf_nodes: u64 = 0;


    // Count each visited leaf node.
    if depth == 0 {
        return 1;
    }
    //let move_list = mg.generate_legal_moves(board, MoveType::All);
    let mut move_list =MoveList::new();
    mg.generate_pseudo_moves(board, &mut move_list, MoveType::All);

    // Run perft for each of the moves.
    for i in 0..move_list.len() {
        // Get the move to be executed and counted.
        let m = move_list.get_move(i);

        // If the move is legal...
        if board.make(m, mg) {
            // Then count the number of leaf nodes it generates...
            leaf_nodes += perft(board, depth - 1, mg);

            // Then unmake the move so the next one can be counted.
            board.unmake();
        }
    }

    // Return the number of leaf nodes for the given position and depth.
    leaf_nodes
}
