//imports
pub mod defs;
mod fen;
mod gamestate;
mod history;
mod utils;

use self::{
    defs::{Pieces, BB_SQUARES},
    gamestate::GameState,
    history::History,
};
use crate::defs::{Bitboard, BoardConsts, Piece, Side, Sides, Square, EMPTY};

use crate::misc::bits;
use std::sync::Arc;

#[derive(Clone)]
pub struct Board {
    pub bb_pieces: [[Bitboard; BoardConsts::PIECE_TYPES]; Sides::BOTH],
    pub bb_side: [Bitboard; Sides::BOTH],
    pub game_state: GameState,
    pub history: History,
    pub piece_list: [Piece; BoardConsts::SQUARES],
}

//impl board struct pub functions
impl Board {
    pub fn new() -> Self {
        Self {
            bb_pieces: [[EMPTY; BoardConsts::PIECE_TYPES]; Sides::BOTH],
            bb_side: [EMPTY; Sides::BOTH],
            game_state: GameState::new(),
            history: History::new(),
            piece_list: [Pieces::NONE; BoardConsts::SQUARES],
        }
    }

    //gets the bitboard for a given piece and side
    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.bb_pieces[side][piece]
    }

    //get side occupancy for a given side
    pub fn occupancy(&self) -> Bitboard {
        self.bb_side[Sides::WHITE] | self.bb_side[Sides::BLACK]
    }

    //get side to move
    pub fn get_curr_side(&self) -> usize {
        self.game_state.active_color as usize
    }

    //get side not moving
    pub fn get_other_side(&self) -> usize {
        (self.game_state.active_color ^ 1) as usize
    }

    //get king square
    pub fn get_king_square(&self, side: Side) -> Square {
        self.bb_pieces[side][Pieces::KING].trailing_zeros() as Square
    }

    //remove piece from the board, for the given side, piece, and square.
    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        //update bitboard
        self.bb_pieces[side][piece] ^= BB_SQUARES[square];
        self.bb_side[side] ^= BB_SQUARES[square];

        //update piece list
        self.piece_list[square] = Pieces::NONE;

        //update game state material

        //update game state piece square table
    }

    //place piece on the board, for the given side, piece, and square.
    pub fn place_piece(&mut self, side: Side, piece: Piece, square: Square) {
        //update bitboard
        self.bb_pieces[side][piece] |= BB_SQUARES[square];
        self.bb_side[side] |= BB_SQUARES[square];

        //update piece list
        self.piece_list[square] = piece;

        //update game state material

        //update game state piece square table
    }

    //move piece on the board, given piece, side, from square, to square
    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.place_piece(side, piece, to);
    }

    //set en passant square
    pub fn set_ep_square(&mut self, square: Square) {
        self.game_state.en_passant = Some(square as u8);
    }

    //clear en passant square
    pub fn clear_ep_square(&mut self) {
        self.game_state.en_passant = None;
    }

    //swap sides
    pub fn swap_side(&mut self) {
        self.game_state.active_color ^= 1;
    }

    // Update castling permissions .
    pub fn update_castling_permissions(&mut self, new_permissions: u8) {
        self.game_state.castling = new_permissions;
    }

    pub fn get_attackable(&self, side: Side) -> Bitboard {
        return self.bb_side[side] & !self.bb_pieces[side][Pieces::KING];
    }
}

// Private board functions (for initializating on startup)
impl Board {
    // Resets/wipes the board. Used by the FEN reader function.
    fn reset(&mut self) {
        self.bb_pieces = [[0; BoardConsts::PIECE_TYPES]; Sides::BOTH];
        self.bb_side = [EMPTY; Sides::BOTH];
        self.game_state = GameState::new();
        self.history.clear();
        self.piece_list = [Pieces::NONE; BoardConsts::SQUARES];
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
    fn init_piece_list(&self) -> [Piece; BoardConsts::SQUARES] {
        let bb_w = self.bb_pieces[Sides::WHITE]; // White piece bitboards
        let bb_b = self.bb_pieces[Sides::BLACK]; // Black piece bitboards
        let mut piece_list: [Piece; BoardConsts::SQUARES] = [Pieces::NONE; BoardConsts::SQUARES];

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
}
