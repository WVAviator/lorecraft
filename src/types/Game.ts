export interface Game {
  id: string;
  name: string;
  summary: GameSummary;
  cover_art: Image;
  narrative: Narrative;
  scenes: Scene[];
  characters: Character[];
  items: Item[];
  title_music: Music;
}

export interface Item {
  name: string;
  description: string;
  image: Image;
}

export interface Character {
  name: string;
  physical_description: string;
  personality: string;
  backstory: string;
  thoughts: string;
  inventory: string[];
  image: Image;
}

export interface Scene {
  name: string;
  narrative: string;
  metadata: string;
  characters: string[];
  items: string[];
  image: Image;
}

export interface Image {
  src: string;
  alt: string;
}

export interface Music {
  src: string;
  metadata: {
    index: number;
    filename: string;
    keywords: string;
    attribution: {
      title: string;
      author: string;
      from: string;
    };
  };
}

export interface GameSummary {
  name: string;
  description: string;
  artStyle: string;
  artTheme: string;
  cover_art: string;
  summary: string;
  winCondition: string;
}

export interface Narrative {
  pages: {
    narrative: string;
    image: Image;
  }[];
}

// export interface PlayerAttribute {
//   name: string;
//   description: string;
//   advancement: string;
//   significance: string;
//   startingValue: number;
// }

// export interface KeyItem {
//   name: string;
//   description: string;
//   location: string;
//   significance: string;
//   image: string;
// }

// export interface KeyArea {
//   name: string;
//   description: string;
//   significance: string;
// }

// export interface KeyCharacter {
//   name: string;
//   description: string;
//   significance: string;
//   image: string;
// }
