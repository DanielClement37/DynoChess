import { MatchState } from '../types/GameState';
import { Move } from '../types/Move';

export const SET_MATCH_STATE = 'SET_MATCH_STATE';
export const SET_MOVE_LIST = 'SET_MOVE_LIST';
export const SET_SELECTED_SQUARE = 'SET_SELECTED_SQUARE';

export interface Action {
    type: "SET_MATCH_STATE" | "SET_MOVE_LIST" | "SET_SELECTED_SQUARE";
    payload: MatchState | Move[] | number | null;
  }