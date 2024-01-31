import { useCallback, useContext } from "react";
import { AppContext } from "../context/AppContext.tsx";
import { BoardTile } from "./BoardTile.tsx";
import { ConvertBitboardsTo64Array, ConvertBitsToMove, ConvertMoveToBits } from "../utils/BoardHelpers.ts";
import { PieceType } from "../types/GameEnums.ts";
import { get_legal_moves, make_engine_move, make_move } from "dyno_engine";
import { MAKE_MOVE, SET_SELECTED_SQUARE } from "../actions/actionTypes.ts";
import { Move } from "../types/Move.ts";
import { BoardState } from "../types/GameState.ts";

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

	const handleMove = async (move: Move) => {
		const moveData = ConvertMoveToBits(move);

		try {
			const response = await make_move(matchState.board, moveData);

			//TODO Fix if statement for terminal conditions because it continues after checkmate and causes an error
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

				// Get new legal moves
				const legalMovesNumbers = get_legal_moves(newBoard);
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

				// AI waits for the board to update before making a move
				const engineMoveResponse = await make_engine_move(newBoard, aiDifficulty as number, newBoard.game_state.active_color);
				console.log(engineMoveResponse);
				if (engineMoveResponse.status === "Checkmate" || engineMoveResponse.status === "PlayerCheckmate" || engineMoveResponse.status === "InvalidMove") {
					// Handle terminal conditions for engine move
					const updatedBoard =
						engineMoveResponse === "Checkmate"
							? engineMoveResponse.Checkmate
							: engineMoveResponse.status === "PlayerCheckmate"
							? engineMoveResponse.PlayerCheckmate
							: newBoard;
					console.log("Terminal condition reached for engine move");
					dispatch({ type: MAKE_MOVE, payload: { board: updatedBoard, aiSettings: matchState.aiSettings } });
				} else {
					// Handle a regular engine move
					const updatedBoard = engineMoveResponse.RegularMove;
					console.log("Regular engine move");
					dispatch({ type: MAKE_MOVE, payload: { board: updatedBoard, aiSettings: matchState.aiSettings } });
				}
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
