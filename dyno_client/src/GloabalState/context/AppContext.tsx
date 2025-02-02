import React, {createContext, useReducer, ReactNode, useEffect} from 'react';
import {AppState, appReducer, initialState} from '../reducers/appReducer.ts';
import {ActionType} from "../actions/actionTypes.ts";
import initDynoEngine, {init_board} from "dyno_engine";

interface IChildren {
    children: ReactNode;
}

interface AppContextType {
    state: AppState;
    dispatch: React.Dispatch<any>;
}

export const AppContext = createContext<AppContextType>({
    state: initialState,
    dispatch: () => null,
});

export const AppContextProvider = ({children}: IChildren) => {
    const [state, dispatch] = useReducer(appReducer, initialState);

    const refreshBoard = async () => {
        const board = await init_board(state.flipBoard);
        dispatch({ type: ActionType.SET_BOARD, payload: board });
    };

    useEffect(() => {
        (async function loadWasmAndBoard() {
            await initDynoEngine();
            await refreshBoard();
        })();
    }, []);

    // When the flipBoard flag changes, reload the board view.
    useEffect(() => {
        refreshBoard();
    }, [state.flipBoard]);

    useEffect(() => {
        (async function loadWasmAndBoard() {
            await initDynoEngine();
            const board = init_board();
            dispatch({type: ActionType.SET_BOARD, payload: board});
        })();
    }, []);

    return <AppContext.Provider value={{state, dispatch}}>{children}</AppContext.Provider>;
};