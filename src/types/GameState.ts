interface GameState {
  current_scene_id: string | null;
  messages: string[];
  inventory: string[];
  character_interaction: CharacterInteraction | null;
  end_game: string | null;
}

interface CharacterInteraction {
  character_id: string;
  messages: CharacterMessage[];
  trade: CharacterTrade | null;
}

interface CharacterMessage {
  text: string;
  is_dialog: boolean;
}

interface CharacterTrade {
  to_player: string | null;
  from_player: string | null;
}
