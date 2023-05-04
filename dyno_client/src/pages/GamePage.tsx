// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import React from 'react';
import { useLocation } from 'react-router-dom';
import {GameSettingsAI} from "../types/GameSettingsAI.ts";


export const GamePage = () => {
    const settings:GameSettingsAI = useLocation().state;

    // TODO: Implement the chess board using the game settings

    return (
        <div>
            <h1>Game Page</h1>
            <p>Difficulty: {settings.difficulty}</p>
            <p>Starting Side: {settings.startingPlayer}</p>
            {/* TODO: Display the chess board */}
        </div>
    );
};