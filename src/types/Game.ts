export interface Game {
  id: string;
  name: string;
  summary: Summary;
  narrative: Narrative[];
}

export interface Summary {
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
    image: string;
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
