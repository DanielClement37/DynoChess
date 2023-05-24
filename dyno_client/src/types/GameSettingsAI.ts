import {Player} from "./GameEnums.ts";

export interface GameSettingsAI{
    difficulty: number;
    startingPlayer: Player | string;
}

