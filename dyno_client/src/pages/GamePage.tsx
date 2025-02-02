// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React, {useContext} from "react";
import GameBoard from "../components/GameBoard.tsx";
import {AppContext} from "../GloabalState/context/AppContext.tsx";
import {ActionType} from "../GloabalState/actions/actionTypes.ts";


export const GamePage = () => {
	const {dispatch} = useContext(AppContext);

	return (
		<div className="game-page">
			<div className="game-body">
				<div className="player-one-info">
					Player Two
				</div>
				<button onClick={() => dispatch({ type: ActionType.FLIP_BOARD })}>
					Flip Board
				</button>
				<GameBoard />
				<div className="player-one-info">
					Player One
				</div>
			</div>
		</div>
	);
};
