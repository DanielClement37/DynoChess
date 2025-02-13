use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PieceOnSquare {
    pub piece_type: u8,  // e.g., 0 for Pawn, 1 for Knight, ...
    pub color: u8,       // 0 or 1
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct BoardView {
    #[serde(with = "BigArray")]
    pub squares: [Option<PieceOnSquare>; 64],
    pub active_color: u8,
    pub castling_rights: u8,
    pub en_passant_square: Option<u8>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub was_flipped: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveView {
    pub from: u8,
    pub to: u8,
    pub piece_type: u8,
    pub capture: u8,
    pub promotion: u8,
}