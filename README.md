# Lorecraft

Lorecraft is a story adventure game generation engine. Plug in your own OpenAI API key and use AI to generate and play entirely unique, custom, and interactive text adventure games.

## How It Works

### Generating a Game

Lorecraft takes in a game idea prompt from you (which can be left empty for a random game idea) and generates a robust framework for a text adventure game through _prompt chaining_.

Each model is given a system prompt that includes detailed expectations regarding content and structure, and then based on each response, other models are then asked to expand on or summarize certain parts of the previous responses. 

The combination of prompts and subsequent generated responses is gradually combined into one big structured JSON file that represents the cohesive structure of the game, including the backstory, lore, the player's goal, characters, key items, and locations.

### Playing the Game

#### The Narrator

When the player starts the game, an AI "Narrator" takes in prompts from the player and returns a response along with any actions that should take place, based on the game summary, level summary, player stats, etc. For example, if the player types, "I examine the old tree" the AI might respond with JSON that looks like this:

```
{
  "message": "As you explore the gnarled branches of the ancient tree, your curiosity leads you to a hidden knothole. Brushing aside the moss, you discover an old key inside, its surface tarnished and etched with intricate patterns. This unexpected find, nestled in the rough inner bark, hints at long-forgotten secrets and stories untold.",
  "actions": [
    "inventory-add 'Old Key'"
  ]
}
```

In this particular case, you would see the message and notice a new item get added to your inventory. Other actions the narrator may perform include actions like 'inventory-remove', 'start-dialog', or 'end-game'.

#### Characters

Characters in the game are fully interactable. During game generation, characters will be created with full profiles, including their backgrounds, personalities, and appearance. 

Once in game, if you request to speak with a character, a separate dialog window will open where you will have a dynamic, interactive conversation with the character. The AI 'Actor' will be fed instructions along the character's profile and other information (like inventory) to conduct this immersive chat with the player.

A player can have really interesting gameplay moments through this feature, as they will be able to develop natural relationships with the characters in the game. Stored conversation summaries can even enable the character to remember previous conversations and provide a truly immersive experience.

### Modifying the Game

Note: This is a stretch feature.

After generating and playing a game, the user may decide that it needs improvement. A built-in editor enables the user to investigate parts of the game's narrative and edit them directly or request that AI regenerate certain parts.

If a user finds a game particularly interesting - all games can be imported and exported, enabling players to send games they have generated to their friends to play.

## Get Involved

This is turning out to be a really ambitious project, and there is a mountain of work that needs done to get the game to a functioning MVP. I would gladly welcome any contributors to assist with this project. I need help building the UI with several different theme options, the backend API calls, state management, and file system interface, and prompt engineering to get high quality responses from the OpenAI models. 

Reach out if you have any experience in Tauri, React, Typescript, Rust, AI prompt engineering, or Photoshop/Figma.
