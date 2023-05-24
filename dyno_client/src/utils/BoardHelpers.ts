import {BoardState, Tile} from "../types/GameState.ts";
import {PieceType, Player} from "../types/GameEnums.ts";

export const InitBoardState = (): BoardState => {
    return {
        bb_pieces: [[ BigInt(16), BigInt(8), BigInt(129), BigInt(36), BigInt(66), BigInt(65280)], [BigInt(1152921504606846976), BigInt(576460752303423488), BigInt(-9151314442816847872), BigInt(2594073385365405696), BigInt(4755801206503243776), BigInt(71776119061217280)]],
        bb_side: [BigInt(65535), BigInt(-281474976710656)],
        game_state: {
            active_color: 0,
            castling: 15,
            halfmove_clock: 0,
            fullmove_number: 1,
            material: [3960, 3960],
            psqt: [-70, -70],
            next_move: 0,
        },
        history: [],
        piece_list: [PieceType.ROOK, PieceType.KNIGHT, PieceType.BISHOP, PieceType.QUEEN, PieceType.KING, PieceType.BISHOP, PieceType.KNIGHT, PieceType.ROOK,
            PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN,
            PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE,
            PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE,
            PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE,
            PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE, PieceType.NONE,
            PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN, PieceType.PAWN,
            PieceType.ROOK, PieceType.KNIGHT, PieceType.BISHOP, PieceType.QUEEN, PieceType.KING, PieceType.BISHOP, PieceType.KNIGHT, PieceType.ROOK
        ]
    }
}

export const ConvertBitboardsTo64Array = (bb_pieces: [[bigint, bigint, bigint, bigint, bigint, bigint], [bigint, bigint, bigint, bigint, bigint, bigint]]): Tile[] => {
    const tiles: Tile[] = [];
    for (let position = 0; position < 64; position++) {
        let pieceFound = false;
        for (let color = 0; color < 2; color++) {
            for (let type = 0; type < 6; type++) {
                const bitboard = bb_pieces[color][type];
                if (((bitboard >> BigInt(position)) & 1n) === 1n) {
                    tiles.push({
                        piece: type,
                        player: color,
                        position: position
                    });
                    pieceFound = true;
                    break;
                }
            }
            if (pieceFound) {
                break;
            }
        }
        if (!pieceFound) {
            tiles.push({
                piece: PieceType.NONE,
                player: Player.NONE,
                position: position
            });
        }
    }
    tiles.sort((a, b) => a.position - b.position);

    return tiles;

}