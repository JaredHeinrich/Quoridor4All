import { readable, writable } from 'svelte/store';

export const playerNames = writable<string[]>(["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"])

export const players = writable<{
  position: {
    x: number,
    y: number,
  },
  wallQuantity: number,
  goal: {
    isXCoordinate: boolean,
    goalLine: number
  },
  playerName: string,
  color: string,
}[]>([
  {
    position: {
      x: 4,
      y: 8,
    },
    wallQuantity: 6,
    goal: {
      isXCoordinate: false,
      goalLine: 0,
    },
    playerName: "Spieler 1",
    color: "red",
  },
  {
    position: {
      x: 0,
      y: 4, 
    },
    wallQuantity: 6,
    goal: {
      isXCoordinate: true,
      goalLine: 8,
    },
    playerName: "spieler 2",
    color: "yellow",
  },
  {
    position: {
      x: 4, 
      y: 0, 
    },
    wallQuantity: 6,
    goal: {
      isXCoordinate: false,
      goalLine: 8,
    },
    playerName: "Spieler 3",
    color: "blue",
  },
  {
    position: {
      x: 8, 
      y: 4, 
    },
    wallQuantity: 6,
    goal: {
      isXCoordinate: true,
      goalLine: 0,
    },
    playerName: "Spieler 4",
    color: "green",
  },
]);

export const walls = writable<{
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}[]>([]);

export const currentPlayerIndex = writable<number>(0);

export const size = readable<number>(9);

export const playerPreviews = writable<{
  position: {
    x: number,
    y: number,
  },
  color: string,
}[]>([]);

export const singlePlayerPreview = writable<{
  position: {
    x: number,
    y: number,
  },
  color: string,
}|null>(null);

export const wallPreview = writable<{
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}|null>(null);

export const gameRunning = writable<boolean>(false);