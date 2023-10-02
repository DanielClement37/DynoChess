import { PieceType } from "./GameEnums.ts";

export interface Move {
    from: number;
    to: number;
    piece: PieceType;
    capture: PieceType;
    promotion: PieceType;
    isCastle: boolean;
    isEnPassant: boolean;
    isDoublePawnPush: boolean;
    sortScore: number;
}