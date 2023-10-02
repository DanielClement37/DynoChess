export enum Player {
    WHITE ,
    BLACK,
    NONE
}

export enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    NONE
}

export class Shift {
    public static PIECE = 0;
    public static FROM_SQ = 3;
    public static TO_SQ = 9;
    public static CAPTURE = 15;
    public static PROMOTION = 18;
    public static EN_PASSANT = 21;
    public static DOUBLE_STEP = 22;
    public static CASTLING = 23;
    public static SORTSCORE = 24;
}