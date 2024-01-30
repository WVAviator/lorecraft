# Lorecraft

Lorecraft is a story adventure game generation engine. Plug in your own OpenAI API key and use AI to generate and play entirely unique, custom, and interactive text adventure games.

## How It Works

### Generating a Game

Lorecraft takes in a game idea prompt from you (which can be left empty for a random game idea) and generates a robust framework for a text adventure game through _prompt chaining_. Each model is given a system prompt that includes detailed expectations regarding content and structure, and then based on each response, other models are then asked to expand on or summarize certain parts of the previous responses through cascading prompts. 

The combination of prompts and subsequent generated responses is gradually combined into one big structured JSON file that represents the cohesive structure of the game, including the backstory, lore, the player's goal, characters, key items, and locations.

Generating games is primarily done through chaining a series of nested abstract factories that each asynchronously generate different types of responses from OpenAI - including form the Image API and Chat Completion API. The interface for working with OpenAI was also developed as a separate crate in the workspace, and is designed using typed builder patterns for each type of request.

### Playing the Game

When playing a game, all of the game state is managed on the Rust side of the application, with any updates passed to the frontend through Tauri events. This removes the need for any kind of complex front-end state management solution, as the frontend just renders what it's told to render and not much more. However, the backend state management is far from complex - most of the interaction with the OpenAI API is done through the Assistant API, which has its own state on OpenAI's servers managed through Threads, Messages, and Runs. So the backend must take in actions from the player as Tauri commands passed from the frontend, decide how to interact with the Assistant API, coordinate its own state with the state of any active OpenAI Threads or Runs (including Runs with function calls), and properly update the game state in response (while notifying the front-end). 

I implemented this through a modified state pattern, as I could not use a typical state pattern due to each state being asynchronous. Essentially, state enums with associated struct variants are mapped to async functions, each of which returns a new state enum variant. It's not super clean, as it technically violates the open-closed principle and DRY, but I could not get a standard state pattern to work with Tokio (probably because async Rust is a state pattern in and of itself). It all currently works, but could use some polishing.

#### The Narrator

When the player starts the game, an AI "Narrator" takes in prompts from the player and returns a response along with any actions that should take place, based on the game summary, level summary, player stats, etc. This is done through the use of OpenAI's new Assistant API - which enables defining "functions" for the AI model to call and respond accordingly. These functions perform in-game actions like moving the player to a new scene, adding items to the player's inventory, or ending the game. Here is an example of a function, defined according to OpenAI's specifications, that triggers an interaction with a character in the game scene:

```
{
  "name": "character_interact",
  "parameters": {
    "type": "object",
    "properties": {
      "character": {
        "type": "string",
        "description": "The character the player wants to interact with. This should match only the name in the character list of the current scene."
      }
    },
    "required": ["character"]
  },
  "description": "Start an interaction between the player and a character in the scene. This should be invoked only when the player directly requests to talk to the character by their description or name."
}
```

When this function is called, the game state is updated to be in a "CharacterInteraction" state, and a chat window pops up alongside a generated image of the character for you to have a conversation.

#### Characters

Characters in the game are fully interactable. During game generation, characters will be created with full profiles, including their backgrounds, personalities, and appearance. 

Once in game, if you request to speak with a character, a separate dialog window will open where you will have a dynamic, interactive conversation with the character. The AI 'Actor' will be fed instructions along with the character's profile and other information (like inventory) to conduct this immersive chat with the player.

A player can have really interesting gameplay moments through this feature, as they will be able to develop natural relationships with the characters in the game. Stored conversation summaries can even enable the character to remember previous conversations and provide a truly immersive experience.

This feature also works with OpenAI's Assistant API, providing functions for the AI character actor to use to give items to the player or to request a trade. Additionally, for smaller less-critical functionality, the AI actor is instructed to use meta-commands like $emotion(happy) or $action(points west) in their replies, which are displayed to the player in italics separately from dialog.

### Modifying the Game

Note: This is a stretch feature.

After generating and playing a game, the user may decide that it needs improvement. A built-in editor enables the user to investigate parts of the game's narrative and edit them directly or request that AI regenerate certain parts.

If a user finds a game particularly interesting - all games can be imported and exported, enabling players to send games they have generated to their friends to play.

## Get Involved

This is turning out to be a really ambitious project, and there is a mountain of work that needs done to get the game to a functioning MVP. I would gladly welcome any contributors to assist with this project. I need help building the UI with several different theme options, the backend API calls, state management, and file system interface, and prompt engineering to get high quality responses from the OpenAI models. 

Primarily the game is hurting for a good high-quality UI/UX right now. If you have React/frontend/design skills, bring it on!

Reach out if you have any experience or passion in Tauri, React, Typescript, Rust, AI prompt engineering, or even Photoshop/Figma.
