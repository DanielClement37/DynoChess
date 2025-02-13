import React, {createContext, useReducer, ReactNode, useEffect} from 'react';
import {AppState, appReducer, initialState} from '../reducers/appReducer.ts';
import {ActionType} from "../actions/actionTypes.ts";
import {BoardView} from "../../types/GameTypes.ts";

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

    useEffect(() => {
        (async () => {
            const engine = await import("../../../public/dyno_engine/dyno_engine.js");
            await engine.default();
            const boardView: BoardView = engine.init_board(true);
            dispatch({type: ActionType.SET_BOARD, payload: boardView});
        })();
    }, []);

    return (
        <AppContext.Provider value={{state, dispatch}}>
            {children}
        </AppContext.Provider>
    );
};