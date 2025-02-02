import {BoardView} from "../../types/GameTypes.ts";
import {ActionType} from "./actionTypes.ts";

export interface SetBoardAction {
    type: ActionType.SET_BOARD;
    payload: BoardView;
}

export interface MakeMoveAction {
    type: ActionType.MAKE_MOVE;
    payload: {
        fromSquare: number;
        toSquare: number;
    };
}

export interface ToggleFlipAction {
    type: ActionType.FLIP_BOARD;
}

export type AppActions = SetBoardAction | MakeMoveAction | ToggleFlipAction;