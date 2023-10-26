// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React, { useContext, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { GameSettingsAI } from "../types/GameSettingsAI.ts";
import { Move } from "../types/Move.ts";
import { Game } from "../components/Game.tsx";
import { PlayerInfo } from "../types/PlayerInfo.ts";
import "../styles/Match.css";
import { make_initial_position, get_legal_moves } from "../../../dyno_engine/pkg";
import { ConvertBitsToMove } from "../utils/BoardHelpers.ts";
import { AppContext } from "../context/AppContext.tsx";

export const GamePage = () => {
	const settings: GameSettingsAI = useLocation().state;
	const { state, dispatch } = useContext(AppContext);
	const matchState = state.matchState;

	useEffect(() => {
		dispatch({
			type: "SET_MATCH_STATE",
			payload: { board: make_initial_position(), aiSettings: settings },
		});
	}, [dispatch, settings]);

	useEffect(() => {
		//get legal moves then convert to move objects
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
	}, [dispatch, matchState.board]);

	// this will end up getting user info from the database
	const player1Info: PlayerInfo = {
		name: "Player 1",
		isHuman: true,
		rating: 800,
	};

	// this will be dynamically generated based on the difficulty
	const player2Info: PlayerInfo = {
		name: "AI",
		isHuman: false,
		rating: 800,
	};

	return (
		<div className="game-page-container">
			<Game player1Info={player1Info} player2Info={player2Info} />
		</div>
	);
};
