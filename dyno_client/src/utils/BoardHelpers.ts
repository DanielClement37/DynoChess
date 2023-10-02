import { Tile } from "../types/GameState.ts";
import { PieceType, Player } from "../types/GameEnums.ts";
import { Move } from "../types/Move.ts";

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
						position: position,
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
				position: position,
			});
		}
	}

	const flippedTiles: Tile[] = [];
	for (let row = 7; row >= 0; row--) {
		for (let col = 0; col < 8; col++) {
			flippedTiles.push(tiles[row * 8 + col]);
		}
	}

	return flippedTiles;
};

/*
Move format explanation

"data" contains all the move information, starting from LSB:

Field       :   bits     Decimal values
============================================
PIECE       :   3        0-7 (use only 0-6)
FROM        :   6        0-63
TO          :   6        0-63
CAPTURE     :   3        0-7 (captured piece)
PROMOTION   :   3        0-7 (piece promoted to)
ENPASSANT   :   1        0-1
DOUBLESTEP  :   1        0-1
CASTLING    :   1        0-1
SORTSCORE   :   16       0-65536


---------------------------------- move data -------------------------------------------
0000000000000000    0        0          0         000       000     000000 000000 000
SORTSCORE           CASTLING DOUBLESTEP ENPASSANT PROMOTION CAPTURE TO     FROM   PIECE
----------------------------------------------------------------------------------------

Field:      PROMOTION   CAPTURE     TO          FROM        PIECE
Bits:       3           3           6           6           3
Shift:      18 bits     15 bits     9 bits      3 bits      0 bits
& Value:    0x7 (7)     0x7 (7)     0x3F (63)   0x3F (63)   0x7 (7)

Field:      SORTSCORE   CASTLING    DOUBLESTEP  ENPASSANT
Bits:       32          1           1           1
Shift:      24 bits     23 bits     22 bits     21 bits
& Value:    0xFFFFFFFF  0x1         0x1 (1)     0x1 (1)

Get the TO field from "data" by:
    -- Shift 9 bits Right
    -- AND (&) with 0x3F

Obviously, storing information in "data" is the other way around.PIECE_NAME
Storing the "To" square: Shift LEFT 9 bits, then XOR with "data".
*/

export const ConvertBitsToMove = (moveData: number): Move => {
	const move: Move = {
		from: 0,
		to: 0,
		piece: 0,
		capture: 0,
		promotion: 0,
		isCastle: false,
		isEnPassant: false,
		isDoublePawnPush: false,
		sortScore: 0,
	};
	move.piece = moveData & 7;
	move.from = (moveData >> 3) & 63;
	move.to = (moveData >> 9) & 63;
	move.capture = (moveData >> 15) & 7;
	move.promotion = (moveData >> 18) & 7;
	move.isEnPassant = ((moveData >> 21) & 1) === 1;
	move.isDoublePawnPush = ((moveData >> 22) & 1) === 1;
	move.isCastle = ((moveData >> 23) & 1) === 1;
	move.sortScore = (moveData >> 24) & 0xffffffff;

	/*
	console.log("Move Data: " + moveData);
	console.log("piece: " + move.piece);
	console.log("from: " + move.from);
	console.log("to: " + move.to);
*/
	return move;
};
