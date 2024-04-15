import { readable, writable } from 'svelte/store';

export const players = writable<{
  position: {
    x: number,
    y: number,
  },
  number_of_available_walls: number,
  goal: {
    is_x_coordinate: boolean,
    goal_line: number
  },
  player_name: string,
  color: string,
}[]>([
  {
    position: {
      x: 4,
      y: 8,
    },
    number_of_available_walls: 6,
    goal: {
      is_x_coordinate: false,
      goal_line: 0,
    },
    player_name: "Player 1",
    color: "red",
  },
  {
    position: {
      x: 0,
      y: 4,
    },
    number_of_available_walls: 6,
    goal: {
      is_x_coordinate: true,
      goal_line: 8,
    },
    player_name: "Player 2",
    color: "yellow",
  },
  {
    position: {
      x: 4,
      y: 0,
    },
    number_of_available_walls: 6,
    goal: {
      is_x_coordinate: false,
      goal_line: 8,
    },
    player_name: "Player 3",
    color: "blue",
  },
  {
    position: {
      x: 8,
      y: 4,
    },
    number_of_available_walls: 6,
    goal: {
      is_x_coordinate: true,
      goal_line: 0,
    },
    player_name: "Player 4",
    color: "green",
  },
]);

export const walls = writable<{
  position: {
    x: number, 
    y: number
  },
  is_horizontal: boolean
}[]>([
  {
    position: {
      x: 5,
      y: 5,
    },
    is_horizontal: true,
  },
  {
    position: {
      x: 0,
      y: 0,
    },
    is_horizontal: false,
  },
  {
    position: {
      x: 7,
      y: 4,
    },
    is_horizontal: true,
  },
  {
    position: {
      x: 1,
      y: 7,
    },
    is_horizontal: false,
  },
  {
    position: {
      x: 4,
      y: 2,
    },
    is_horizontal: true,
  },
]);

export const currentPlayerIndex = writable<number>(0);

export const size = readable<number>(9);

export const playerPreviews = writable<{
  position: {
    x: number,
    y: number,
  },
  color: string,
}[]>([]);

export const wallPreview = writable<{
  position: {
    x: number, 
    y: number
  },
  is_horizontal: boolean
}>();