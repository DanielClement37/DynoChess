import { SET_MATCH_STATE, SET_MOVE_LIST, SET_SELECTED_SQUARE } from "../actions/actionTypes";
import { make_initial_position } from "engine_wasm";
import { MatchState } from "../types/GameState";
import { Move } from "../types/Move";

export interface AppState {
	matchState: MatchState;
	moveList: Move[];
	selectedSquare: number;
}

export const initialState: AppState = {
	matchState: { board: make_initial_position(), aiSettings: null },
	moveList: [],
	selectedSquare: -1,
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const appReducer = (state: AppState, action: any) => {
	switch (action.type) {
		case SET_MATCH_STATE:
			return { ...state, matchState: action.payload };
		case SET_MOVE_LIST:
			return { ...state, moveList: action.payload };
		case SET_SELECTED_SQUARE:
			return { ...state, selectedSquare: action.payload };
		default:
			return state;
	}
};
