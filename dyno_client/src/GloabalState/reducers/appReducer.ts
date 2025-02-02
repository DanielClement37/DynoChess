import {BoardView} from "../../types/GameTypes.ts";
import {ActionType} from "../actions/actionTypes.ts";
import {make_move_js} from  "dyno_engine"

export interface AppState {
	currentBoard: BoardView | null;
	flipBoard: boolean;
}

export const initialState: AppState = {
	currentBoard: null,
	flipBoard: true,
};

export const appReducer = (state: AppState, action: any) => {
	switch (action.type) {

		case ActionType.SET_BOARD: {
			const newBoard = action.payload;
			return { ...state, currentBoard: newBoard };
		}

		case ActionType.MAKE_MOVE: {
			const { fromSquare, toSquare } = action.payload;

			if (state.currentBoard) {
				const response = make_move_js(
					state.currentBoard,
					fromSquare,
					toSquare
				);
				if (response.status === "RegularMove") {
					return { ...state, currentBoard: response.board };
				}
			}
			return state;
		}
		case ActionType.FLIP_BOARD:
			return { ...state, flipBoard: !state.flipBoard };

		default:
			return state;
	}
};
