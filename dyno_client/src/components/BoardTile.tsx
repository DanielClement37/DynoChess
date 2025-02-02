import React from 'react';
import { PieceOnSquare } from '../types/GameTypes.ts';
import blackBishop from '../assets/bishop-black.svg';
import whiteBishop from '../assets/bishop-white.svg';
import blackKing from '../assets/king-black.svg';
import whiteKing from '../assets/king-white.svg';
import blackKnight from '../assets/knight-black.svg';
import whiteKnight from '../assets/knight-white.svg';
import blackPawn from '../assets/pawn-black.svg';
import whitePawn from '../assets/pawn-white.svg';
import blackQueen from '../assets/queen-black.svg';
import whiteQueen from '../assets/queen-white.svg';
import blackRook from '../assets/rook-black.svg';
import whiteRook from '../assets/rook-white.svg';

interface BoardTileProps {
    index: number;
    square: PieceOnSquare | null;
}

const BoardTile: React.FC<BoardTileProps> = ({ index, square }) => {

    const row = Math.floor(index / 8);
    const col = index % 8;

    const isDarkSquare = (row + col) % 2 === 1;

    // This function returns the corresponding SVG image for a piece
    function getPieceImage(square: PieceOnSquare | null): string | null {
        if (!square) return null; // No piece
        const { piece_type, color } = square;
        // color 0=white, 1=black
        const isWhite = (color === 0);

        switch (piece_type) {
            case 0: // King
                return isWhite ? whiteKing : blackKing;
            case 1: // Queen
                return isWhite ? whiteQueen : blackQueen;
            case 2: // Rook
                return isWhite ? whiteRook : blackRook;
            case 3: // Bishop
                return isWhite ? whiteBishop : blackBishop;
            case 4: // Knight
                return isWhite ? whiteKnight : blackKnight;
            case 5: // Pawn
                return isWhite ? whitePawn : blackPawn;
            default:
                return null;
        }
    }

    const pieceSrc = getPieceImage(square);

    return (
        <div className={`board-tile ${isDarkSquare ? 'dark-square' : 'light-square'}`}>
            {pieceSrc && <img src={pieceSrc} alt="chess-piece" className="piece-img" />}
        </div>
    );
};

export default BoardTile;