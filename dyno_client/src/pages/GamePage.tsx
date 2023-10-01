// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React from 'react';
import { useLocation } from 'react-router-dom';
import {GameSettingsAI} from "../types/GameSettingsAI.ts";
import {Game} from "../components/Game.tsx";
import {MatchState} from "../types/GameState.ts";
import {PlayerInfo} from "../types/PlayerInfo.ts";
import "../styles/Match.css"
import {make_initial_position, get_legal_moves} from "../../../dyno_engine/pkg";
import {InitBoardState} from "../utils/BoardHelpers.ts";


export const GamePage = () => {
    const settings:GameSettingsAI = useLocation().state;
    const initialPosition = make_initial_position();
    const [matchState, setMatchState] = React.useState<MatchState>({board: initialPosition, aiSettings:settings});

    const generateMovesHandler = () => {
        const moves = get_legal_moves(matchState.board);
        console.log(moves);
    }


    // this will end up getting user info from the database
    const player1Info: PlayerInfo = {
        name: "Player 1",
        isHuman: true,
        rating: 800
    }

    // this will be dynamically generated based on the difficulty
    const player2Info: PlayerInfo = {
        name: "AI",
        isHuman: false,
        rating: 800
    }

    return (
        <div className="game-page-container">
            <Game  boardState={matchState.board} player1Info={player1Info} player2Info={player2Info}/>
            <button onClick={generateMovesHandler}>Generate Moves</button>
        </div>
    );
};