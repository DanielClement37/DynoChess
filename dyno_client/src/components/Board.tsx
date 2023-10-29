import { useCallback, useContext } from "react";
import { AppContext } from "../context/AppContext.tsx";
import { BoardTile } from "./BoardTile.tsx";
import { ConvertBitboardsTo64Array, ConvertBitsToMove, ConvertMoveToBits, FlipSquare } from "../utils/BoardHelpers.ts";
import { PieceType } from "../types/GameEnums.ts";
import { get_legal_moves, make_engine_move } from "dyno_engine";
import { MAKE_MOVE, SET_SELECTED_SQUARE } from "../actions/actionTypes.ts";
import { Move } from "../types/Move.ts";

export const Board = () => {
	const { state, dispatch } = useContext(AppContext);
	const { moveList, selectedSquare, matchState } = state;
	const aiDifficulty = matchState.aiSettings?.difficulty;
	const tiles = ConvertBitboardsTo64Array(matchState.board.bb_pieces);

	// Function to filter moves originating from the selected square
	const getPossibleMoves = useCallback(() => {
		return moveList.filter((move) => move.from === selectedSquare);
	}, [moveList, selectedSquare]);

	const possibleMoves = getPossibleMoves();

	// Function to handle the piece movement
	const handleMove = (to: number) => {
		//try move in wasm engine then update board
		const move_string = `${FlipSquare(selectedSquare)} ${FlipSquare(to)}`;

		if (aiDifficulty === undefined) throw new Error("AI difficulty is undefined");
		try {
			const response = make_engine_move(matchState.board, move_string, aiDifficulty);
			console.log(response);
			if (response === "Checkmate") {
				// Handle checkmate
				const newBoard = response.RegularMove;
				console.log("Checkmate reached");
				dispatch({ type: MAKE_MOVE, payload: { board: newBoard, aiSettings: matchState.aiSettings } });
			} else if (response.status === "PlayerCheckmate") {
				// Handle player checkmate
				const newBoard = response.RegularMove;
				console.log("Player checkmate reached");
				dispatch({ type: MAKE_MOVE, payload: { board: newBoard, aiSettings: matchState.aiSettings } });
			} else if (response.status === "InvalidMove") {
				// Handle an invalid move
				console.log("Invalid move");
			} else {
				// Handle a regular move
				const newBoard = response.RegularMove;
				console.log("Regular move");
				dispatch({ type: MAKE_MOVE, payload: { board: newBoard, aiSettings: matchState.aiSettings } });
			}
		} catch (error) {
			console.log(error);
		}

		// Update the selected square
		dispatch({ type: SET_SELECTED_SQUARE, payload: -1 });

		//get new legal moves
		const legalMovesNumbers = get_legal_moves(matchState.board);
		const moveDataList: number[] = legalMovesNumbers.list.map((moveDataObj: { data: number }) => moveDataObj.data);
		const moveCount: number = legalMovesNumbers.count;
		const moves: Move[] = moveDataList.slice(0, moveCount).map((moveData: number) => {
			const move = ConvertBitsToMove(moveData);
			return move;
		});
		dispatch({
			type: "SET_MOVE_LIST",
			payload: moves,
		});
	};

	return (
		<div className="board-container">
			{tiles.map((tile, index) => {
				const tileColor = ((index % 8) + Math.floor(index / 8)) % 2 === 0 ? "light" : "dark";

				const isPossibleMove = possibleMoves.some((move) => move.to === index);

				const handleTileClick = () => {
					if (isPossibleMove) {
						handleMove(index);
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
