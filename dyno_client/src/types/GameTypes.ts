export interface BoardView {
    squares: Array<PieceOnSquare | null>;
    active_color: number;
    castling_rights: number;
    en_passant_square: number | null;
    halfmove_clock: number;
    fullmove_number: number;
    is_checkmate: boolean;
    is_stalemate: boolean;
    was_flipped: boolean;
}

export interface PieceOnSquare {
    piece_type: number; // 0=KING,1=QUEEN,2=ROOK,3=BISHOP,4=KNIGHT,5=PAWN (depends on your Rust)
    color: number;      // 0=White,1=Black
}

export interface MoveView{
    from: number,
    to: number,
    piece_type: number,
    capture: number,
    promotion: number,
}

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