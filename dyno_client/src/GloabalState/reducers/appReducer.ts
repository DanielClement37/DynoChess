import {BoardView} from "../../types/GameTypes.ts";
import {ActionType} from "../actions/actionTypes.ts";
export interface AppState {
	currentBoard: BoardView | null;
}

export const initialState: AppState = {
	currentBoard: null,
};

export const appReducer = (state: AppState, action: any) => {
	switch (action.type) {
		case ActionType.SET_BOARD: {
			const newBoard = action.payload;
			return { ...state, currentBoard: newBoard };
		}
		default:
			return state;
	}
};
