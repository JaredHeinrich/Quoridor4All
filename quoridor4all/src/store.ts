import { readable, writable } from 'svelte/store';

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
    playerName: "Player 1",
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
    playerName: "Player 2",
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
    playerName: "Player 3",
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
    playerName: "Player 4",
    color: "green",
  },
]);

export const walls = writable<{
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}[]>([
  {
    position: {
      x: 5,
      y: 5,
    },
    isHorizontal: true,
  },
  {
    position: {
      x: 0,
      y: 0,
    },
    isHorizontal: false,
  },
  {
    position: {
      x: 7,
      y: 4,
    },
    isHorizontal: true,
  },
  {
    position: {
      x: 1,
      y: 7,
    },
    isHorizontal: false,
  },
  {
    position: {
      x: 4,
      y: 2,
    },
    isHorizontal: true,
  },
  {
    position: {
      x: 3,
      y: 5,
    },
    isHorizontal: false,
  }
]);

export const currentPlayerIndex = writable<number>(0);

export const size = readable<number>(9);

export const playerPreviews = writable<{
  position: {
    x: number,
    y: number,
  },
  color: string,
  isVisible: boolean,
}[]>([]);

export const wallPreview = writable<{
  position: {
    x: number, 
    y: number
  },
  isHorizontal: boolean
}>();