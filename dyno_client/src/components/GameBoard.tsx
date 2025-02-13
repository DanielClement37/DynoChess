import React, {useContext, useState} from 'react';
import { AppContext } from '../GloabalState/context/AppContext.tsx';
import BoardTile from './BoardTile.tsx';
import "../styles/GameBoard.css"
import {ActionType} from "../GloabalState/actions/actionTypes.ts";
import {MoveView} from "../types/GameTypes.ts";


const GameBoard = () => {
    const { state, dispatch } = useContext(AppContext);
    const squares = state.currentBoard?.squares || [];

    const [selectedSquare, setSelectedSquare] = useState<number | null>(null);
    const [highlightedMoves, setHighlightedMoves] = useState<number[]>([]);

    async function attemptMove(fromSquare: number, toSquare: number) {
        if (!state.currentBoard) return;
        const engine = await import("../../public/dyno_engine/dyno_engine.js");
        await engine.default();
        const response = engine.make_move_js(
            state.currentBoard,
            fromSquare,
            toSquare
        );
        if (response.status === "RegularMove") {
            dispatch({ type: ActionType.SET_BOARD, payload: response.board });
        } else if (response.status === "InvalidMove") {
            console.log("Invalid move");
        } else if (response.status === "Checkmate") {
            console.log("Checkmate!");
            dispatch({ type: ActionType.SET_BOARD, payload: response.board });
        }
    }

    async function handleTileClick(index: number) {
        if (selectedSquare === null) {
            if (squares[index] !== null && squares[index]?.color === state.currentBoard?.active_color) {
                setSelectedSquare(index);
                await showMovesForSquare(index);
            }
        } else {
            const alreadySelectedColor = squares[selectedSquare]?.color;
            const newSquareColor = squares[index]?.color;
            if (newSquareColor !== undefined && newSquareColor === alreadySelectedColor) {
                setSelectedSquare(index);
                await showMovesForSquare(index);
            } else {
                await attemptMove(selectedSquare, index);
                setSelectedSquare(null);
                setHighlightedMoves([]);
            }
        }
    }

    async function showMovesForSquare(index: number) {
        if (!state.currentBoard) return;

        const engine = await import("../../public/dyno_engine/dyno_engine.js");
        await engine.default();
        const moves:MoveView[] = engine.get_legal_moves_js(state.currentBoard);
        const possibleDestinations = moves.filter(mv => mv.from === index).map(mv => mv.to);
        setHighlightedMoves(possibleDestinations);
    }

    return (
        <div className="game-board-container">
            <div className="game-board">
                {squares.map((square, i) => {
                    const isHighlighted = highlightedMoves.includes(i);

                    return (
                        <BoardTile
                            key={i}
                            index={i}
                            square={square}
                            isSelected={selectedSquare === i}
                            isHighlighted={isHighlighted}
                            onClick={() => handleTileClick(i)}
                        />
                    );
                })}
            </div>
        </div>
    );
};

export default GameBoard;