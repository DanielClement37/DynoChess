import { useCallback, useContext } from "react";
import { AppContext } from "../context/AppContext.tsx";
import { BoardTile } from "./BoardTile.tsx";
import { ConvertBitboardsTo64Array } from "../utils/BoardHelpers.ts";
import { PieceType } from "../types/GameEnums.ts";

export const Board = () => {
	const { state, dispatch } = useContext(AppContext);
	const { moveList, selectedSquare, matchState } = state;
	const tiles = ConvertBitboardsTo64Array(matchState.board.bb_pieces);

	// Function to filter moves originating from the selected square
	const getPossibleMoves = useCallback(() => {
		return moveList.filter((move) => move.from === selectedSquare);
	}, [moveList, selectedSquare]);

	const possibleMoves = getPossibleMoves();

	return (
		<div className="board-container">
			{tiles.map((tile, index) => {
				const tileColor = ((index % 8) + Math.floor(index / 8)) % 2 === 0 ? "light" : "dark";

				const isPossibleMove = possibleMoves.some((move) => move.to === index);

				const handleTileClick = () => {
					// Dispatch an action to update the selected square when clicked
					dispatch({ type: "SET_SELECTED_SQUARE", payload: index });
				};

				return (
					<BoardTile
						key={index}
						isPossibleMove={isPossibleMove}
						hasPiece={tile.piece !== PieceType.NONE}
						piece={tile.piece}
						color={tile.player}
						tileColor={tileColor}
						selectSquareHandler={handleTileClick}
					/>
				);
			})}
		</div>
	);
};
