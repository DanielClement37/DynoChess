// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React, {useContext} from "react";
import GameBoard from "../components/GameBoard.tsx";
import {AppContext} from "../GloabalState/context/AppContext.tsx";
import {ActionType} from "../GloabalState/actions/actionTypes.ts";


export const GamePage = () => {
	const {state,dispatch} = useContext(AppContext);
	const currentPlayer = state.currentBoard?.active_color === 0? "white" : "black";


	async function handleFlip() {
		if (!state.currentBoard) return;
		const engine = await import("../../public/dyno_engine/dyno_engine.js");
		await engine.default();
		const newView = await engine.flip_board_js(state.currentBoard);
		dispatch({ type: ActionType.SET_BOARD, payload: newView });
	}

	async function handleReset() {
		if (!state.currentBoard) return;
		const engine = await import("../../public/dyno_engine/dyno_engine.js");
		await engine.default();
		const currentFlip = state.currentBoard.was_flipped;
		const newBoard = engine.init_board(currentFlip);
		dispatch({ type: ActionType.SET_BOARD, payload: newBoard });
	}

	return (
		<div className="game-page">
			<div className="game-body">
				<h2>{currentPlayer} to move</h2>
				<div className="player-one-info">
					Player Two
				</div>
				<button onClick={handleFlip}>
					Flip Board
				</button>
				<button onClick={handleReset}>
					Reset Board
				</button>
				<GameBoard />
				<div className="player-one-info">
					Player One
				</div>
			</div>
		</div>
	);
};
