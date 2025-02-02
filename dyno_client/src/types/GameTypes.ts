/**
 * Mirrors `frontend::BoardView` in Rust.
 * Make sure field names match exactly.
 */
export interface BoardView {
    // squares[0..63] each can be null or a PieceOnSquare
    squares: Array<PieceOnSquare | null>;

    active_color: number;      // 0 = White, 1 = Black
    castling_rights: number;   // bitmask for castling
    en_passant_square: number | null; // optional
    halfmove_clock: number;
    fullmove_number: number;

    // For UI info, if you want them
    is_checkmate: boolean;
    is_stalemate: boolean;
}

export interface PieceOnSquare {
    piece_type: number; // 0=KING,1=QUEEN,2=ROOK,3=BISHOP,4=KNIGHT,5=PAWN (depends on your Rust)
    color: number;      // 0=White,1=Black
}

// A union type:
export type EngineResponse =
    | {
    status: "RegularMove";
    board: BoardView;
}
    | {
    status: "Checkmate";
    board: BoardView;
}
    | {
    status: "PlayerCheckmate";
    board: BoardView;
}
    | {
    status: "InvalidMove";
    board: BoardView;
}
    | {
    status: "EnginePanicked";
}