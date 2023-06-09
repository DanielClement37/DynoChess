#![allow(dead_code)]
#![allow(unused_variables)]

pub type Bitboard = u64;
pub type Piece = usize;
pub type Side = usize;
pub type Square = usize;

#[derive(Copy, Clone, PartialEq)]
pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct BoardConsts;
impl BoardConsts {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct Castling;
impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
    pub const ALL: u8 = 15;
}

pub const EMPTY: u64 = 0;
pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_LEGAL_MOVES: u8 = 255;
pub const MAX_DEPTH: u8 = 254;
pub const MAX_MOVE_RULE: u8 = 100; // 50/75 move rule