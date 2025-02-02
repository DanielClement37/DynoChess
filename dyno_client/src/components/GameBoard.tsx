import React, { useContext } from 'react';
import { AppContext } from '../GloabalState/context/AppContext.tsx';
import BoardTile from './BoardTile.tsx';
import "../styles/GameBoard.css"
const GameBoard = () => {
    const { state } = useContext(AppContext);

    // Typically, squares are 64 elements. We'll check if we have them.
    const squares = state.currentBoard?.squares || [];

    return (
        <div className="game-board-container">
            <div className="game-board">
                {squares.map((square, index) => (
                    <BoardTile
                        key={index}
                        index={index}
                        square={square}
                    />
                ))}
            </div>
        </div>
    );
};

export default GameBoard;