#![allow(dead_code)]
#![allow(unused_variables)]

pub mod defs;
mod fen;
mod gamestate;
mod history;
mod playmove;
mod utils;
mod zobrist;

use self::{
    defs::{Pieces, BB_SQUARES},
    gamestate::GameState,
    history::History,
    zobrist::{ZobristKey, ZobristRandoms},
};
use crate::frontend::{BoardView, PieceOnSquare};
use crate::{
    defs::{Bitboard, NrOf, Piece, Side, Sides, Square, EMPTY},
    eval::{
        defs::PIECE_VALUES,
        material,
        psqt::{self, FLIP, PSQT_MG},
    },
    misc::bits,
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::sync::Arc;

// This file implements the engine's board representation; it is bit-board
// based, with the least significant bit being A1.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]; Sides::BOTH],
    pub bb_side: [Bitboard; Sides::BOTH],
    pub game_state: GameState,
    pub history: History,
    #[serde(with = "BigArray")]
    pub piece_list: [Piece; NrOf::SQUARES],
    zr: Arc<ZobristRandoms>,
}

// Public functions for use by other modules.
impl Board {
    // Creates a new board with either the provided FEN, or the starting position.
    pub fn new() -> Self {
        Self {
            bb_pieces: [[EMPTY; NrOf::PIECE_TYPES]; Sides::BOTH],
            bb_side: [EMPTY; Sides::BOTH],
            game_state: GameState::new(),
            history: History::new(),
            piece_list: [Pieces::NONE; NrOf::SQUARES],
            zr: Arc::new(ZobristRandoms::new()),
        }
    }

    // Return a bitboard with locations of a certain piece type for one of the sides.
    pub fn get_pieces(&self, piece: Piece, side: Side) -> Bitboard {
        self.bb_pieces[side][piece]
    }

    // Return a bitboard containing all the pieces on the board.
    pub fn occupancy(&self) -> Bitboard {
        self.bb_side[Sides::WHITE] | self.bb_side[Sides::BLACK]
    }

    // Returns the side to move.
    pub fn get_curr_side(&self) -> usize {
        self.game_state.active_color as usize
    }

    // Returns the side that is NOT moving.
    pub fn get_other_side(&self) -> usize {
        (self.game_state.active_color ^ 1) as usize
    }

    // Returns the square the king is currently on.
    pub fn king_square(&self, side: Side) -> Square {
        self.bb_pieces[side][Pieces::KING].trailing_zeros() as Square
    }

    // Remove a piece from the board, for the given side, piece, and square.
    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bb_pieces[side][piece] ^= BB_SQUARES[square];
        self.bb_side[side] ^= BB_SQUARES[square];
        self.piece_list[square] = Pieces::NONE;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        self.game_state.material[side] -= PIECE_VALUES[piece];

        let flip = side == Sides::WHITE;
        let s = if flip { FLIP[square] } else { square };
        self.game_state.psqt[side] -= PSQT_MG[piece][s] as i16;
    }

    // Put a piece onto the board, for the given side, piece, and square.
    pub fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bb_pieces[side][piece] |= BB_SQUARES[square];
        self.bb_side[side] |= BB_SQUARES[square];
        self.piece_list[square] = piece;
        self.game_state.zobrist_key ^= self.zr.piece(side, piece, square);

        // Incremental updates
        // =============================================================
        self.game_state.material[side] += PIECE_VALUES[piece];

        let flip = side == Sides::WHITE;
        let s = if flip { FLIP[square] } else { square };
        self.game_state.psqt[side] += PSQT_MG[piece][s] as i16;
    }

    // Remove a piece from the from-square, and put it onto the to-square.
    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    // Set a square as being the current ep-square.
    pub fn set_ep_square(&mut self, square: Square) {
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = Some(square as u8);
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
    }

    // Clear the ep-square. (If the ep-square is None already, nothing changes.)
    pub fn clear_ep_square(&mut self) {
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = None;
        self.game_state.zobrist_key ^= self.zr.en_passant(self.game_state.en_passant);
    }

    // Swap side from WHITE <==> BLACK
    pub fn swap_side(&mut self) {
        self.game_state.zobrist_key ^= self.zr.side(self.game_state.active_color as usize);
        self.game_state.active_color ^= 1;
        self.game_state.zobrist_key ^= self.zr.side(self.game_state.active_color as usize);
    }

    // Update castling permissions and take Zobrist-key into account.
    pub fn update_castling_permissions(&mut self, new_permissions: u8) {
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
        self.game_state.castling = new_permissions;
        self.game_state.zobrist_key ^= self.zr.castling(self.game_state.castling);
    }

    pub fn to_board_view(&self, flip: bool) -> BoardView {
        let mut squares: [Option<PieceOnSquare>; 64] = [None; 64];

        for visual_index in 0..64 {
            let (row, col) = (visual_index / 8, visual_index % 8);
            let engine_index = if flip {
                row * 8 + (7 - col)
            } else {
                (7 - row) * 8 + col
            };

            let piece_type = self.piece_list[engine_index];
            if piece_type != Pieces::NONE {
                let color = if (self.bb_side[Sides::WHITE] & BB_SQUARES[engine_index]) != 0 {
                    Sides::WHITE
                } else {
                    Sides::BLACK
                };

                squares[visual_index] = Some(PieceOnSquare {
                    piece_type: piece_type as u8,
                    color: color as u8,
                });
            }
        }

        BoardView {
            squares,
            active_color: self.game_state.active_color,
            castling_rights: self.game_state.castling,
            en_passant_square: self.game_state.en_passant,
            halfmove_clock: self.game_state.halfmove_clock as u16,
            fullmove_number: self.game_state.fullmove_number,
            is_checkmate: false,
            is_stalemate: false,
            was_flipped: flip, 
        }
    }

    pub fn from_board_view(view: BoardView) -> Board {
        let mut board = Board::new();
        board.reset();
        let was_flipped = view.was_flipped;
        for visual_index in 0..64 {
            if let Some(piece_on_sq) = view.squares[visual_index] {
                let (row, col) = (visual_index / 8, visual_index % 8);
                let engine_index = if view.was_flipped {
                    row * 8 + (7 - col)
                } else {
                    (7 - row) * 8 + col
                };
                let piece_type = piece_on_sq.piece_type as usize;
                let side = piece_on_sq.color as usize;
                board.put_piece(side, piece_type, engine_index);
            }
        }

        // Copy over additional game_state fields from BoardView
        board.game_state.active_color = view.active_color;
        board.game_state.castling = view.castling_rights;
        board.game_state.en_passant = view.en_passant_square;
        board.game_state.halfmove_clock = view.halfmove_clock as u8;
        board.game_state.fullmove_number = view.fullmove_number;

        // Reinitialize material, psqt, Zobrist, etc.
        board.init();

        board
    }
}

// Private board functions (for initializating on startup)
impl Board {
    // Resets/wipes the board. Used by the FEN reader function.
    fn reset(&mut self) {
        self.bb_pieces = [[0; NrOf::PIECE_TYPES]; Sides::BOTH];
        self.bb_side = [EMPTY; Sides::BOTH];
        self.game_state = GameState::new();
        self.history.clear();
        self.piece_list = [Pieces::NONE; NrOf::SQUARES];
    }

    // Main initialization function. This is used to initialize the "other"
    // bit-boards that are not set up by the FEN-reader function.
    fn init(&mut self) {
        // Gather all the pieces of a side into one bitboard; one bitboard
        // with all the white pieces, and one with all black pieces.
        let pieces_per_side_bitboards = self.init_pieces_per_side_bitboards();
        self.bb_side[Sides::WHITE] = pieces_per_side_bitboards.0;
        self.bb_side[Sides::BLACK] = pieces_per_side_bitboards.1;

        // Initialize the piece list, zobrist key, and material count. These will
        // later be updated incrementally.
        self.piece_list = self.init_piece_list();
        self.game_state.zobrist_key = self.init_zobrist_key();

        let material = material::count(self);
        self.game_state.material[Sides::WHITE] = material.0;
        self.game_state.material[Sides::BLACK] = material.1;

        let psqt = psqt::apply(self);
        self.game_state.psqt[Sides::WHITE] = psqt.0;
        self.game_state.psqt[Sides::BLACK] = psqt.1;
    }

    // Gather the pieces for each side into their own bitboard.
    fn init_pieces_per_side_bitboards(&self) -> (Bitboard, Bitboard) {
        let mut bb_white: Bitboard = 0;
        let mut bb_black: Bitboard = 0;

        // Iterate over the bitboards of every piece type.
        for (bb_w, bb_b) in self.bb_pieces[Sides::WHITE]
            .iter()
            .zip(self.bb_pieces[Sides::BLACK].iter())
        {
            bb_white |= *bb_w;
            bb_black |= *bb_b;
        }

        // Return a bitboard with all white pieces, and a bitboard with all
        // black pieces.
        (bb_white, bb_black)
    }

    // Initialize the piece list. This list is used to quickly determine
    // which piece type (rook, knight...) is on a square without having to
    // loop through the piece bitboards.
    fn init_piece_list(&self) -> [Piece; NrOf::SQUARES] {
        let bb_w = self.bb_pieces[Sides::WHITE]; // White piece bitboards
        let bb_b = self.bb_pieces[Sides::BLACK]; // Black piece bitboards
        let mut piece_list: [Piece; NrOf::SQUARES] = [Pieces::NONE; NrOf::SQUARES];

        // piece_type is enumerated, from 0 to 6.
        // 0 = KING, 1 = QUEEN, and so on, as defined in board::defs.
        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            let mut white_pieces = *w; // White pieces of type "piece_type"
            let mut black_pieces = *b; // Black pieces of type "piece_type"

            // Put white pieces into the piece list.
            while white_pieces > 0 {
                let square = bits::next(&mut white_pieces);
                piece_list[square] = piece_type;
            }

            // Put black pieces into the piece list.
            while black_pieces > 0 {
                let square = bits::next(&mut black_pieces);
                piece_list[square] = piece_type;
            }
        }

        piece_list
    }

    // Initialize the zobrist hash. This hash will later be updated incrementally.
    fn init_zobrist_key(&self) -> ZobristKey {
        // Keep the key here.
        let mut key: u64 = 0;

        // Same here: "bb_w" is shorthand for
        // "self.bb_pieces[Sides::WHITE]".
        let bb_w = self.bb_pieces[Sides::WHITE];
        let bb_b = self.bb_pieces[Sides::BLACK];

        // Iterate through all piece types, for both white and black.
        // "piece_type" is enumerated, and it'll start at 0 (KING), then 1
        // (QUEEN), and so on.
        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            // Assume the first iteration; piece_type will be 0 (KING). The
            // following two statements will thus get all the pieces of
            // type "KING" for white and black. (This will obviously only
            // be one king, but with rooks, there will be two in the
            // starting position.)
            let mut white_pieces = *w;
            let mut black_pieces = *b;

            // Iterate through all the piece locations of the current piece
            // type. Get the square the piece is on, and then hash that
            // square/piece combination into the zobrist key.
            while white_pieces > 0 {
                let square = bits::next(&mut white_pieces);
                key ^= self.zr.piece(Sides::WHITE, piece_type, square);
            }

            // Same for black.
            while black_pieces > 0 {
                let square = bits::next(&mut black_pieces);
                key ^= self.zr.piece(Sides::BLACK, piece_type, square);
            }
        }

        // Hash the castling, active color, and en-passant state into the key.
        key ^= self.zr.castling(self.game_state.castling);
        key ^= self.zr.side(self.game_state.active_color as usize);
        key ^= self.zr.en_passant(self.game_state.en_passant);

        // Done; return the key.
        key
    }
}
