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
	move.from = FlipSquare(move.from);
	move.to = (moveData >> 9) & 63;
	move.to = FlipSquare(move.to);
	move.capture = (moveData >> 15) & 7;
	move.promotion = (moveData >> 18) & 7;
	move.isEnPassant = ((moveData >> 21) & 1) === 1;
	move.isDoublePawnPush = ((moveData >> 22) & 1) === 1;
	move.isCastle = ((moveData >> 23) & 1) === 1;
	move.sortScore = (moveData >> 24) & 0xffffffff;

	return move;
};


//flip y then flip x axis
export const FlipSquare = (square: number): number => {
	const row = square >> 3;
	const col = square & 7;
	return (7 - row) * 8 + col;	
}

export const flipXAxis = (square: number): number => {
	const row = square >> 3;
	const col = square & 7;
	return row * 8 + (7 - col);
}

export const ConvertMoveToBits = (move: Move): number => {

	//flip the col of the square
	move.from = flipXAxis(move.from);
	move.to = flipXAxis(move.to);

	let moveData = 0;
	moveData |= move.piece;
	moveData |= (63 - move.from) << 3;
	moveData |= (63 - move.to) << 9;
	moveData |= move.capture << 15;
	moveData |= move.promotion << 18;
	moveData |= (move.isEnPassant ? 1 : 0) << 21;
	moveData |= (move.isDoublePawnPush ? 1 : 0) << 22;
	moveData |= (move.isCastle ? 1 : 0) << 23;
	moveData |= move.sortScore << 24;
	return moveData;
};