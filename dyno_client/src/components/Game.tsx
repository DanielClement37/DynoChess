import {BoardState} from "../types/GameState.ts";
import {Board} from "./Board.tsx";
import {GameInfoPanel} from "./GameInfoPanel.tsx";
import {GamePlayerInfo} from "./GamePlayerInfo.tsx";
import {PlayerInfo} from "../types/PlayerInfo.ts";

export interface GameProps {
    boardState: BoardState;
    player1Info: PlayerInfo;
    player2Info: PlayerInfo;

}

export const Game = ({boardState, player1Info, player2Info} : GameProps) => {

    return (
        <div>
            <div className="board-container">
                <GamePlayerInfo playerInfo={player1Info}/>
                <Board boardState={boardState}/>
                <GamePlayerInfo playerInfo={player2Info}/>
            </div>
            <div className="game-info-container">
                <GameInfoPanel />
            </div>
        </div>
    );
};