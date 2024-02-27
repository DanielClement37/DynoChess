import { useCallback, useContext } from "react";
import { AppContext } from "../context/AppContext.tsx";
import { BoardTile } from "./BoardTile.tsx";
import { ConvertBitboardsTo64Array, ConvertBitsToMove, ConvertMoveToBits } from "../utils/BoardHelpers.ts";
import { PieceType } from "../types/GameEnums.ts";
import { make_move } from "dyno_engine";
import { MAKE_MOVE, SET_SELECTED_SQUARE } from "../actions/actionTypes.ts";
import { Move } from "../types/Move.ts";
import { BoardState } from "../types/GameState.ts";

export const Board = () => {
	const { state, dispatch } = useContext(AppContext);
	const { moveList, selectedSquare, matchState } = state;
	const tiles = ConvertBitboardsTo64Array(matchState.board.bb_pieces);

	// Function to filter moves originating from the selected square
	const getPossibleMoves = useCallback(() => {
		return moveList.filter((move) => move.from === selectedSquare);
	}, [moveList, selectedSquare]);

	const possibleMoves = getPossibleMoves();

	const handleMove = async (move: Move) => {
		const moveData = ConvertMoveToBits(move);

		try {
			const response = await make_move(matchState.board, moveData);

			if (response.status === "Checkmate" || response.status === "PlayerCheckmate" || response.status === "InvalidMove") {
				// Handle terminal conditions
				const newBoard = response === "Checkmate" ? response.Checkmate : response.status === "PlayerCheckmate" ? response.PlayerCheckmate : matchState.board;
				console.log("Terminal condition reached");
				dispatch({ type: MAKE_MOVE, payload: { board: newBoard, aiSettings: matchState.aiSettings } });
				dispatch({ type: SET_SELECTED_SQUARE, payload: -1 });
			} else {
				// Handle a regular move
				const newBoard = response.RegularMove as BoardState;
				console.log("Regular move");
				dispatch({ type: MAKE_MOVE, payload: { board: newBoard, aiSettings: matchState.aiSettings } });
				dispatch({ type: SET_SELECTED_SQUARE, payload: -1 });

				// Allow React to re-render before proceeding
				await new Promise(resolve => setTimeout(resolve, 0));
			}
		} catch (error) {
			console.log(error);
		}
	};

	return (
		<div className="board-container">
			{tiles.map((tile, index) => {
				const tileColor = ((index % 8) + Math.floor(index / 8)) % 2 === 0 ? "light" : "dark";

				const isPossibleMove = possibleMoves.some((move) => move.to === index);

				const handleTileClick = () => {
					if (isPossibleMove) {
						//get move object from possible moves if move is promotion then handle promotion with a modal then handle move
						const move = possibleMoves.find((move) => move.to === index);
						if (move?.promotion !== PieceType.NONE) {
							//TODO handle promotion
							console.log("promotion");
						} else {
							handleMove(move as Move);
						}
					} else {
						// Dispatch an action to update the selected square when clicked
						dispatch({ type: SET_SELECTED_SQUARE, payload: index });
					}
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
