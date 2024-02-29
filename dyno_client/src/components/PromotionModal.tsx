// PromotionModal.tsx
import { PieceType, Player } from "../types/GameEnums.ts";
import { classNames } from "../utils/ClassNames.ts";
import BlackPawn from "../assets/pawn-black.svg";
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

export interface PromotionModalProps {
    color: Player;
    onSelect: (pieceType: PieceType) => void;
    column: number;
    
}

export const PromotionModal = ({ color, onSelect, column }: PromotionModalProps) => {
  const pieces = [PieceType.QUEEN, PieceType.ROOK, PieceType.BISHOP, PieceType.KNIGHT];
 

  //get the image for the pieces
    const getPieceImage = (pieceType: PieceType) => {
        switch (pieceType) {
            case PieceType.PAWN:
            return color === Player.WHITE ? WhitePawn : BlackPawn;
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
            return "";
        }
    };

  return (
    <div className="promotion-modal">
      {pieces.map((pieceType) => (
        <div key={pieceType} className="promotion-option" onClick={() => onSelect(pieceType)}>
          <img src={getPieceImage(pieceType)} alt=""/>
        </div>
      ))}
    </div>
  );
};