import {BoardState} from "../types/GameState.ts";
import {BoardTile} from "./BoardTile.tsx";
import {PieceType} from "../types/GameEnums.ts";
import {ConvertBitboardsTo64Array} from "../utils/BoardHelpers.ts";

export interface BoardProps {
    boardState: BoardState;
}

export const Board = ({boardState}:BoardProps) => {
    const tiles = ConvertBitboardsTo64Array(boardState.bb_pieces);

    return (
        <div className="board-container">
            {
                tiles.map((tile, index) => {
                    const tileColor = (index % 8 + Math.floor(index / 8)) % 2 === 0 ? "light" : "dark";
                    return <BoardTile
                        key={index}
                        hasPiece={tile.piece !== PieceType.NONE}
                        piece={tile.piece}
                        color={tile.player}
                        tileColor={tileColor}
                    />
                })
            }
        </div>
    );
};