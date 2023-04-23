import React, { useState } from 'react';
import { Link } from 'react-router-dom';

interface Settings {
    difficulty: number;
    startingSide: string;
}


export const HomePage = () => {
    const [showSettings, setShowSettings] = useState(false);
    const [settings, setSettings] = useState<Settings>({
        difficulty: 1,
        startingSide: 'white',
    });


    const handlePlayClick = () => {
        setShowSettings(true);
    };

    const handleDifficultyChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSettings({ ...settings, difficulty: Number(e.target.value) });
    };

    const handleStartingSideChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setSettings({ ...settings, startingSide: e.target.value });
    };

    const renderSettingsForm = () => {
        if (showSettings) {

            return (
                <div>
                    <h2>Game Settings</h2>
                    <div>
                        <label>
                            Difficulty:
                            <input
                                type="number"
                                value={settings.difficulty}
                                onChange={handleDifficultyChange}
                            />
                        </label>
                        <label>
                            Starting Side:
                            <select value={settings.startingSide} onChange={handleStartingSideChange}>
                                <option value="white">White</option>
                                <option value="black">Black</option>
                                <option value="random">Random</option>
                            </select>
                        </label>
                    </div>
                    <Link to={{ pathname: '/play/ai', state: { settings } }}><button>Start Game</button></Link>
                </div>
            );
        }
        return null;
    };

    const renderPlayButton = () => {
        if (!showSettings) {
            return (
                <button onClick={handlePlayClick}>Play vs AI</button>
            );
        }
        return null;
    };

    return (
        <div>
            <h1>Home Page</h1>
            {renderPlayButton()}
            {renderSettingsForm()}
        </div>
    );
};