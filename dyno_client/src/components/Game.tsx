import {BoardState} from "../types/GameState.ts";
import {Board} from "./Board.tsx";
import {GameInfoPanel} from "./GameInfoPanel.tsx";
import {GamePlayerInfo} from "./GamePlayerInfo.tsx";
import {PlayerInfo} from "../types/PlayerInfo.ts";


export interface GameProps {
    player1Info: PlayerInfo;
    player2Info: PlayerInfo;

}

export const Game = ({ player1Info, player2Info} : GameProps) => {

    return (
        <div className="game-container">
            <div className="game-board-container">
                <GamePlayerInfo playerInfo={player2Info}/>
                <Board />
                <GamePlayerInfo playerInfo={player1Info}/>
            </div>
            <div className="game-info-container">
                <GameInfoPanel />
            </div>
        </div>
    );
};