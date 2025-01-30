export interface AppState {

}

export const initialState: AppState = {
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const appReducer = (state: AppState, action: any) => {
	switch (action.type) {
		default:
			return state;
	}
};
