use crate::{
    board::defs::{Pieces, PIECE_NAME, SQUARE_NAME},
    misc::print,
};
use crate::defs::{Side, Sides};

#[derive(Clone, Copy)]
pub struct GameState {
    pub active_color: u8,
    pub castling: u8,
    pub halfmove_clock: u8,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
    //pub next_move: Move,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_color: 0,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            //next_move: Move::init(0),
        }
    }
    fn get_qs_castle_rights(&self, side:Side) ->bool{
        return match side {
            Sides::WHITE => self.castling & 0x1 != 0,
            Sides::BLACK => self.castling & 0x4 != 0,
            _ => false
        }
    }
    fn get_ks_castle_rights(&self, side:Side) ->bool{
        return match side {
            Sides::WHITE => self.castling & 0x2 != 0,
            Sides::BLACK => self.castling & 0x8 != 0,
            _ => false
        }
    }

   pub fn white_can_castle_qs(&self) ->bool{
        return if !self.get_qs_castle_rights(Sides::WHITE) {
            false
        } else {
            true
        }

    }
    pub fn white_can_castle_ks(&self) ->bool{
        return if !self.get_ks_castle_rights(Sides::WHITE) {
            false
        } else {
            true
        }

    }
    pub fn black_can_castle_qs(&self) ->bool{
        return if !self.get_qs_castle_rights(Sides::BLACK) {
            false
        } else {
            true
        }
    }
    pub fn black_can_castle_ks(&self) ->bool{
        return if !self.get_qs_castle_rights(Sides::BLACK) {
            false
        } else {
            true
        }
    }

    pub fn to_string(&self) -> String {
        let ep = if let Some(x) = self.en_passant {
            SQUARE_NAME[x as usize]
        } else {
            "-"
        };
        

        format!(
            "ac: {} cperm: {} ep: {} hmc: {} fmn: {} ",
            self.active_color,
            print::castling_as_string(self.castling),
            ep,
            self.halfmove_clock,
            self.fullmove_number,

        )
    }
}