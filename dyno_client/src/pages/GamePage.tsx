// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React from 'react';
import { useLocation } from 'react-router-dom';
import {GameSettingsAI} from "../types/GameSettingsAI.ts";
import {Game} from "../components/Game.tsx";
import {MatchState} from "../types/GameState.ts";
import {InitBoardState} from "../utils/BoardHelpers.ts";
import {PlayerInfo} from "../types/PlayerInfo.ts";



export const GamePage = () => {
    const settings:GameSettingsAI = useLocation().state;
    const [matchState, setMatchState] = React.useState<MatchState>({board: InitBoardState(), aiSettings:settings});
    console.log(matchState);

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
        <div>
            <Game  boardState={matchState.board} player1Info={player1Info} player2Info={player2Info}/>
        </div>
    );
};