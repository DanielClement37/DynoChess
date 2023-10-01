use crate::{
    board::defs::{Pieces, PIECE_NAME, SQUARE_NAME},
    defs::Sides,
    misc::print,
    movegen::defs::Move,
};

use serde::{Serialize, Deserialize};

// This is simply a struct that collects all the variables holding the game sate.
// It makes it very easy to make a backup of the game state during make(), and
// restore it when performing unmake(). It prevents having to backup and restore
// each game state variable one by one.

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub active_color: u8,
    pub castling: u8,
    pub halfmove_clock: u8,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
    pub zobrist_key: u64,
    pub material: [u16; Sides::BOTH],
    pub psqt: [i16; Sides::BOTH],
    pub next_move: Move,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_color: 0,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            zobrist_key: 0,
            material: [0; Sides::BOTH],
            psqt: [0; Sides::BOTH],
            next_move: Move::new(0),
        }
    }

    pub fn as_string(&self) -> String {
        let ep = if let Some(x) = self.en_passant {
            SQUARE_NAME[x as usize]
        } else {
            "-"
        };

        let promotion = if self.next_move.promoted() != Pieces::NONE {
            PIECE_NAME[self.next_move.promoted()]
        } else {
            ""
        };

        format!(
            "zk: {:x} ac: {} cperm: {} ep: {} hmc: {} fmn: {} mat: {}/{}, psqt: {}/{} next: {}{}{}",
            self.zobrist_key,
            self.active_color,
            print::castling_as_string(self.castling),
            ep,
            self.halfmove_clock,
            self.fullmove_number,
            self.material[Sides::WHITE],
            self.material[Sides::BLACK],
            self.psqt[Sides::WHITE],
            self.psqt[Sides::BLACK],
            SQUARE_NAME[self.next_move.from()],
            SQUARE_NAME[self.next_move.to()],
            promotion
        )
    }
}
