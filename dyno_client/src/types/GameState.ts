import {GameSettingsAI} from "./GameSettingsAI.ts";
import {PieceType, Player} from "./GameEnums.ts";

export interface BoardState {
    bb_pieces: [[bigint, bigint, bigint, bigint,bigint, bigint],[bigint, bigint, bigint, bigint,bigint, bigint]],
    bb_side: [bigint, bigint],
    game_state: GameState,
    history: GameState[],
    piece_list: number[],
}

export interface GameState {
    active_color: number,
    castling: number,
    halfmove_clock: number,
    en_passant?: number,
    fullmove_number: number,
    material: [number, number],
    psqt: [number, number],
    next_move: number,
}

export interface MatchState {
    board: BoardState
    aiSettings: GameSettingsAI | null
}

export interface Tile {
    piece: PieceType,
    player: Player
    position: number
}