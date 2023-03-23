#![allow(dead_code)]
#![allow(unused_variables)]

mod create;
pub mod defs;
mod init;
mod magics;
mod movelist;

use crate::{
    board::{
        defs::{Pieces, Squares, BB_RANKS, BB_SQUARES},
        Board,
    },
    defs::{Bitboard, Castling, NrOf, Piece, Side, Sides, Square, EMPTY},
    misc::bits,
};
use defs::{Move, MoveType, Shift};
use magics::Magic;
use movelist::MoveList;

/**
 * @brief Size of moves to pre-reserve before generating moves.
 *
 * At the current time 218 seems to be an upper bound on the maximum number
 * of moves from any one position.
 */
// This is a list of all pieces a pawn can promote to.
const PROMOTION_PIECES: [usize; 4] = [Pieces::QUEEN, Pieces::ROOK, Pieces::BISHOP, Pieces::KNIGHT];

pub const ROOK_TABLE_SIZE: usize = 102_400; // Total permutations of all rook blocker boards.
pub const BISHOP_TABLE_SIZE: usize = 5_248; // Total permutations of all bishop blocker boards.

#[derive(Clone)]
pub struct MoveGen {
    king: [Bitboard; NrOf::SQUARES],
    knight: [Bitboard; NrOf::SQUARES],
    pawns: [[Bitboard; NrOf::SQUARES]; Sides::BOTH],
    rook: Vec<Bitboard>,
    bishop: Vec<Bitboard>,
    rook_magics: [Magic; NrOf::SQUARES],
    bishop_magics: [Magic; NrOf::SQUARES],
}

//public methods
impl MoveGen {
    /**
     * @brief Constructs a new MoveGen and generates moves for the given board.
     *
     * @param board Board to generate moves for.
     */
    pub fn new() -> Self {
        let magics: Magic = Default::default();
        let mut mg = Self {
            king: [EMPTY; NrOf::SQUARES],
            knight: [EMPTY; NrOf::SQUARES],
            pawns: [[EMPTY; NrOf::SQUARES]; Sides::BOTH],
            rook: vec![EMPTY; ROOK_TABLE_SIZE],
            bishop: vec![EMPTY; BISHOP_TABLE_SIZE],
            rook_magics: [magics; NrOf::SQUARES],
            bishop_magics: [magics; NrOf::SQUARES],
        };
        mg.init_king();
        mg.init_knight();
        mg.init_pawns();
        mg.init_magics(Pieces::ROOK);
        mg.init_magics(Pieces::BISHOP);
        return mg;
    }

    pub fn generate_legal_moves(&self, board: &Board, mt: MoveType) -> MoveList{
        let mut pseudo_moves = MoveList::new();
        let mut legal_moves_list = MoveList::new();
        let mut temp_board = board.clone();
        self.generate_pseudo_moves(board, &mut pseudo_moves, mt);

        for i in 0..pseudo_moves.len() {
            let m = pseudo_moves.get_move(i);
            let is_legal = temp_board.make(m,&self);

            if is_legal {
                legal_moves_list.push(m);
                temp_board.unmake();
            }
        }
        return legal_moves_list;
    }

    // Generates moves for the side that is to move. The MoveType parameter
    // determines if all moves, or only captures need to be generated.
    pub fn generate_pseudo_moves(&self, board: &Board, ml: &mut MoveList, mt: MoveType) {
        self.piece(board, Pieces::KING, ml, mt);
        self.piece(board, Pieces::KNIGHT, ml, mt);
        self.piece(board, Pieces::ROOK, ml, mt);
        self.piece(board, Pieces::BISHOP, ml, mt);
        self.piece(board, Pieces::QUEEN, ml, mt);
        self.pawns(board, ml, mt);

        if mt == MoveType::All || mt == MoveType::Quiet {
            self.castling(board, ml);
        }
    }

    // Return non-slider (King, Knight) attacks for the given square.
    pub fn get_non_slider_attacks(&self, piece: Piece, square: Square) -> Bitboard {
        match piece {
            Pieces::KING => self.king[square],
            Pieces::KNIGHT => self.knight[square],
            _ => panic!("Not a king or a knight: {}", piece),
        }
    }
    // Return slider attacsk for Rook, Bishop and Queen using the magic numbers.
    pub fn get_slider_attacks(
        &self,
        piece: Piece,
        square: Square,
        occupancy: Bitboard,
    ) -> Bitboard {
        match piece {
            Pieces::ROOK => {
                let index = self.rook_magics[square].get_index(occupancy);
                self.rook[index]
            }
            Pieces::BISHOP => {
                let index = self.bishop_magics[square].get_index(occupancy);
                self.bishop[index]
            }
            Pieces::QUEEN => {
                let r_index = self.rook_magics[square].get_index(occupancy);
                let b_index = self.bishop_magics[square].get_index(occupancy);
                self.rook[r_index] ^ self.bishop[b_index]
            }
            _ => panic!("Not a sliding piece: {}", piece),
        }
    }

    // Return pawn attacks for the given square.
    pub fn get_pawn_attacks(&self, side: Side, square: Square) -> Bitboard {
        self.pawns[side][square]
    }
}

impl MoveGen {
    pub fn piece(&self, board: &Board, piece: Piece, list: &mut MoveList, mt: MoveType) {
        let curr_side = board.get_curr_side();
        let bb_occupancy = board.occupancy();

        // Get squares that are empty, occupied by our own pieces, and occupied by
        // our opponent's pieces.
        let bb_empty = !bb_occupancy;
        let bb_own_pieces = board.bb_side[curr_side];
        let bb_opponent_pieces = board.bb_side[board.get_other_side()];

        let mut bb_pieces = board.get_pieces(piece, curr_side);

        // Generate moves for each piece of the type passed into the function.
        while bb_pieces > 0 {
            let from = bits::next(&mut bb_pieces);
            let bb_target = match piece {
                Pieces::KING | Pieces::KNIGHT => self.get_non_slider_attacks(piece, from),
                Pieces::QUEEN | Pieces::ROOK | Pieces::BISHOP => {
                    self.get_slider_attacks(piece, from, bb_occupancy)
                }
                _ => panic!("Not a piece: {}", piece),
            };

            // Generate moves according to requested move type.
            let bb_moves = match mt {
                MoveType::All => bb_target & !bb_own_pieces,
                MoveType::Quiet => bb_target & bb_empty,
                MoveType::Capture => bb_target & bb_opponent_pieces,
            };

            self.add_move(board, piece, from, bb_moves, list);
        }
    }

    pub fn pawns(&self, board: &Board, list: &mut MoveList, mt: MoveType) {
        const UP: i8 = 8;
        const DOWN: i8 = -8;

        // Create shorthand variables.
        let curr_side = board.get_curr_side();
        let bb_opponent_pieces = board.bb_side[board.get_other_side()];
        let bb_empty = !board.occupancy();
        let bb_fourth = BB_RANKS[Board::fourth_rank(curr_side)];
        let direction = if curr_side == Sides::WHITE { UP } else { DOWN };
        let rotation_count = (NrOf::SQUARES as i8 + direction) as u32;
        let mut bb_pawns = board.get_pieces(Pieces::PAWN, curr_side);

        // As long as there are pawns, generate moves for each of them.
        while bb_pawns > 0 {
            let from = bits::next(&mut bb_pawns);
            let to = (from as i8 + direction) as usize;
            let mut bb_moves = 0;

            // Generate pawn pushes
            if mt == MoveType::All || mt == MoveType::Quiet {
                let bb_push = BB_SQUARES[to];
                let bb_one_step = bb_push & bb_empty;
                let bb_two_step = bb_one_step.rotate_left(rotation_count) & bb_empty & bb_fourth;
                bb_moves |= bb_one_step | bb_two_step;
            }

            // Generate pawn captures
            if mt == MoveType::All || mt == MoveType::Capture {
                let bb_targets = self.get_pawn_attacks(curr_side, from);
                let bb_captures = bb_targets & bb_opponent_pieces;
                let bb_ep_capture = match board.game_state.en_passant {
                    Some(ep) => bb_targets & BB_SQUARES[ep as usize],
                    None => 0,
                };
                bb_moves |= bb_captures | bb_ep_capture;
            }

            self.add_move(board, Pieces::PAWN, from, bb_moves, list);
        }
    }

    pub fn castling(&self, board: &Board, list: &mut MoveList) {
        // Create shorthand variables.
        let curr_side = board.get_curr_side();
        let other_side = board.get_other_side();
        let castle_perms_white = (board.game_state.castling & (Castling::WK | Castling::WQ)) > 0;
        let castle_perms_black = (board.game_state.castling & (Castling::BK | Castling::BQ)) > 0;
        let bb_occupancy = board.occupancy();
        let mut bb_king = board.get_pieces(Pieces::KING, curr_side);
        let from = bits::next(&mut bb_king);

        // Generate castling moves for white.
        if curr_side == Sides::WHITE && castle_perms_white {
            // Kingside
            if board.game_state.castling & Castling::WK > 0 {
                let bb_kingside_blockers = BB_SQUARES[Squares::F1] | BB_SQUARES[Squares::G1];
                let is_kingside_blocked = (bb_occupancy & bb_kingside_blockers) > 0;

                if !is_kingside_blocked
                    && !self.square_attacked(board, other_side, Squares::E1)
                    && !self.square_attacked(board, other_side, Squares::F1)
                {
                    let to = BB_SQUARES[from] << 2;
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }

            if board.game_state.castling & Castling::WQ > 0 {
                // Queenside
                let bb_queenside_blockers =
                    BB_SQUARES[Squares::B1] | BB_SQUARES[Squares::C1] | BB_SQUARES[Squares::D1];
                let is_queenside_blocked = (bb_occupancy & bb_queenside_blockers) > 0;

                if !is_queenside_blocked
                    && !self.square_attacked(board, other_side, Squares::E1)
                    && !self.square_attacked(board, other_side, Squares::D1)
                {
                    let to = BB_SQUARES[from] >> 2;
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
        }

        // Generate castling moves for black.
        if curr_side == Sides::BLACK && castle_perms_black {
            // Kingside
            if board.game_state.castling & Castling::BK > 0 {
                let bb_kingside_blockers = BB_SQUARES[Squares::F8] | BB_SQUARES[Squares::G8];
                let is_kingside_blocked = (bb_occupancy & bb_kingside_blockers) > 0;

                if !is_kingside_blocked
                    && !self.square_attacked(board, other_side, Squares::E8)
                    && !self.square_attacked(board, other_side, Squares::F8)
                {
                    let to = BB_SQUARES[from] << 2;
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }

            // Queenside
            if board.game_state.castling & Castling::BQ > 0 {
                let bb_queenside_blockers =
                    BB_SQUARES[Squares::B8] | BB_SQUARES[Squares::C8] | BB_SQUARES[Squares::D8];
                let is_queenside_blocked = (bb_occupancy & bb_queenside_blockers) > 0;

                if !is_queenside_blocked
                    && !self.square_attacked(board, other_side, Squares::E8)
                    && !self.square_attacked(board, other_side, Squares::D8)
                {
                    let to = BB_SQUARES[from] >> 2;
                    self.add_move(board, Pieces::KING, from, to, list);
                }
            }
        }
    }

    // Add the generated moves to the move list.
    pub fn add_move(
        &self,
        board: &Board,
        piece: Piece,
        from: Square,
        to: Bitboard,
        list: &mut MoveList,
    ) {
        // Shorthand variiables.
        let mut bb_to = to;
        let curr_side = board.get_curr_side();
        let promotion_rank = Board::promotion_rank(curr_side);
        let is_pawn = piece == Pieces::PAWN;

        // As long as there are still to-squres in bb_to, this piece has moves to add.
        while bb_to > 0 {
            // More shorthand variables
            let to_square = bits::next(&mut bb_to);
            let capture = board.piece_list[to_square];
            let en_passant = match board.game_state.en_passant {
                Some(square) => is_pawn && (square as usize == to_square),
                None => false,
            };
            let promotion = is_pawn && Board::is_square_on_rank(to_square, promotion_rank);
            let double_step = is_pawn && ((to_square as i8 - from as i8).abs() == 16);
            let castling = (piece == Pieces::KING) && ((to_square as i8 - from as i8).abs() == 2);

            // Gather all data for this move into one 64-bit integer.
            let mut move_data = (piece)
                | from << Shift::FROM_SQ
                | to_square << Shift::TO_SQ
                | capture << Shift::CAPTURE
                | (en_passant as usize) << Shift::EN_PASSANT
                | (double_step as usize) << Shift::DOUBLE_STEP
                | (castling as usize) << Shift::CASTLING;

            // Push the move to the piece list...
            if !promotion {
                move_data |= Pieces::NONE << Shift::PROMOTION;
                list.push(Move::new(move_data));
            } else {
                // ...or push four promotion moves.
                PROMOTION_PIECES.iter().for_each(|piece| {
                    let promotion_piece = *piece << Shift::PROMOTION;
                    list.push(Move::new(move_data | promotion_piece))
                });
            }
        }
    }
}


impl MoveGen {
    #[cfg_attr(debug_assertions, inline(never))]
    #[cfg_attr(not(debug_assertions), inline(always))]
    // Determine if a square is attacked by 'attacker', on the given board.
    pub fn square_attacked(&self, board: &Board, attacker: Side, square: Square) -> bool {
        let attackers = board.bb_pieces[attacker];

        // Use the super-piece method: get the moves for each piece,
        // starting from the given square. This provides the sqaures where
        // a piece has to be, to be able to reach the given square.
        let occupancy = board.occupancy();
        let bb_king = self.get_non_slider_attacks(Pieces::KING, square);
        let bb_rook = self.get_slider_attacks(Pieces::ROOK, square, occupancy);
        let bb_bishop = self.get_slider_attacks(Pieces::BISHOP, square, occupancy);
        let bb_knight = self.get_non_slider_attacks(Pieces::KNIGHT, square);
        let bb_pawns = self.get_pawn_attacks(attacker ^ 1, square);
        let bb_queen = bb_rook | bb_bishop;

        // Then determine if such a piece is actually there: see if a rook
        // is on one of the squares a rook has to be to reach the given
        // square. Same for the queen, knight, etc... As soon as one is
        // found, the square is attacked.
        (bb_king & attackers[Pieces::KING] > 0)
            || (bb_rook & attackers[Pieces::ROOK] > 0)
            || (bb_queen & attackers[Pieces::QUEEN] > 0)
            || (bb_bishop & attackers[Pieces::BISHOP] > 0)
            || (bb_knight & attackers[Pieces::KNIGHT] > 0)
            || (bb_pawns & attackers[Pieces::PAWN] > 0)
    }
}