import { PlayerInfo } from "../types/PlayerInfo.ts";

export interface GamePlayerInfoProps {
	playerInfo: PlayerInfo;
}

export const GamePlayerInfo = ({ playerInfo }: GamePlayerInfoProps) => {
	return (
		<div className="player-info-container">
			<div className="player-name">{playerInfo.name}</div>
			<div className="player-rating">{playerInfo.rating}</div>
		</div>
	);
};
