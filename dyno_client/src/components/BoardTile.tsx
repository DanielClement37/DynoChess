import {PieceType, Player} from "../types/GameEnums.ts";
import {classNames} from "../utils/ClassNames.ts";
import PawnBlack from "../assets/pawn-black.svg";
import WhitePawn from "../assets/pawn-white.svg";
import BlackRook from "../assets/rook-black.svg";
import WhiteRook from "../assets/rook-white.svg";
import BlackKnight from "../assets/knight-black.svg";
import WhiteKnight from "../assets/knight-white.svg";
import BlackBishop from "../assets/bishop-black.svg";
import WhiteBishop from "../assets/bishop-white.svg";
import BlackQueen from "../assets/queen-black.svg";
import WhiteQueen from "../assets/queen-white.svg";
import BlackKing from "../assets/king-black.svg";
import WhiteKing from "../assets/king-white.svg";



export interface BoardTileProps {
    piece:number;
    color: Player;
    tileColor: string;
    hasPiece: boolean;
}
export const BoardTile = ({piece, color, tileColor,hasPiece}: BoardTileProps) => {
    // get the image for the pieces
    const getPieceImage = () => {
        switch (piece) {
            case PieceType.PAWN:
                return color === Player.WHITE ? WhitePawn : PawnBlack;
            case PieceType.ROOK:
                return color === Player.WHITE ? WhiteRook : BlackRook;
            case PieceType.KNIGHT:
                return color === Player.WHITE ? WhiteKnight : BlackKnight;
            case PieceType.BISHOP:
                return color === Player.WHITE ? WhiteBishop : BlackBishop;
            case PieceType.QUEEN:
                return color === Player.WHITE ? WhiteQueen : BlackQueen;
            case PieceType.KING:
                return color === Player.WHITE ? WhiteKing : BlackKing;
            case PieceType.NONE:
            default:
                return '';
        }
    }

    if(hasPiece) {
        return (
            //display the tile and the piece
            <div className={classNames(
                "board-tile",
                tileColor == "light" ? 'light' : 'dark',
            )}>
                <img src={getPieceImage()} alt="piece"/>
            </div>
        );
    } else{
        return(
            <div className={classNames(
                "board-tile",
                tileColor == "light" ? 'light' : 'dark',
            )}>
                <h5>empty</h5>
            </div>
        );
    }
};