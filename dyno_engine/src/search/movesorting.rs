use crate::board::defs::Pieces;
use crate::defs::{NrOf};
use crate::movegen::defs::{ MoveList};

const MAX_SORT_SCORE: u32 = u32::MAX;
const MVV_LVA_OFFSET: u32 = 10000; // Ensure this is higher than any MVV-LVA score
const KILLER_MOVE_SCORE: u32 = 9000; // Below MVV-LVA_OFFSET but above any other move
const BASE_MOVE_SCORE: u32 = 0; // Default score for moves not fitting above categories

pub const MVV_LVA: [[u16; NrOf::PIECE_TYPES + 1]; NrOf::PIECE_TYPES + 1] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim K, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];

pub fn score_moves(moves:&mut MoveList) -> &MoveList {
    for i in 0..moves.len() {
        let mut m = moves.get_move(i);

        if m.captured() != Pieces::NONE{
            // Use MVV-LVA table to score captures
            m.set_sort_score( MVV_LVA[m.captured() as usize][m.piece() as usize] as u32 + MVV_LVA_OFFSET);
        } else {
            // Base score for other moves, possibly modified by history heuristic
            m.set_sort_score( BASE_MOVE_SCORE); // Or use history heuristic here
        }
    }

    // Sort moves based on sort_score, descending
    for i in 0..moves.len() {
        let mut best_index = i;
        let mut best_score = 0;
        for j in i..moves.len() {
            if moves.get_move(j).get_sort_score() > best_score {
                best_score = moves.get_move(j).get_sort_score();
                best_index = j;
            }
        }
        moves.swap(i as usize, best_index as usize);
    }
    return moves
}