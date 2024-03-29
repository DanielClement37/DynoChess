import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Player } from "../types/GameEnums.ts";
import { GameSettingsAI } from "../types/GameSettingsAI.ts";

export const HomePage = () => {
	const [showModal, setShowModal] = useState(false);
	const [difficulty, setDifficulty] = useState(1);
	const [startingPlayer, setStartingPlayer] = useState<Player | string>(Player.WHITE);
	const navigate = useNavigate();

	const handlePlayClick = () => {
		setShowModal(true);
	};

	// eslint-disable-next-line @typescript-eslint/ban-ts-comment
	// @ts-ignore
	const handleFormSubmit = (event) => {
		event.preventDefault();
		setShowModal(false);

		let finalStartingPlayer: Player | string = startingPlayer;
		if (startingPlayer === "random") {
			finalStartingPlayer = Math.random() < 0.5 ? Player.WHITE : Player.BLACK;
		}

		navigate("/play/ai", { state: {  startingPlayer: finalStartingPlayer } as GameSettingsAI });
	};

	return (
		<div>
			<button onClick={handlePlayClick}>Play AI</button>
			{showModal && (
				<form onSubmit={handleFormSubmit}>
					<label>
						Starting Player:
						<select value={startingPlayer} onChange={(event) => setStartingPlayer(event.target.value as Player | string)}>
							<option value={Player.WHITE}>White</option>
							<option value={Player.BLACK}>Black</option>
						</select>
					</label>
					<br />
					<button type="submit">Start Game</button>
				</form>
			)}
		</div>
	);
};
