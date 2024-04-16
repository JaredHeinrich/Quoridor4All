import { isAfterThisSquare, isInThisSquare } from "./coordinateCalculation";
import { players, currentPlayerIndex, size, walls, playerPreviews, wallPreview, singlePlayerPreview } from "../../store"
import { get } from 'svelte/store';

const UP: { x: number, y: number } = { x: 0, y: -1 }; 
const DOWN: { x: number, y: number } = { x: 0, y: +1 }; 
const LEFT: { x: number, y: number } = { x: -1, y: 0 }; 
const RIGTH: { x: number, y: number } = { x: +1, y: 0 }; 

function getPossibleNextPawnPositons(): { x: number, y: number }[] {
  let playerPosition = get(players)[get(currentPlayerIndex)].position;

  //all surrounding positions are possible moveDirections at first
  let possibleMoveDirections = [
    UP, DOWN, LEFT, RIGTH
  ];
  let possibleMovePositions: any = [];

  possibleMoveDirections.forEach((possibleMoveDirection) => {
    let possiblePosition = {
      x: playerPosition.x + possibleMoveDirection.x,
      y: playerPosition.y + possibleMoveDirection.y,
    };

    if (positionOutOfBoard(possiblePosition)) {
      return;
    }

    if (noWallObstacle(playerPosition, possibleMoveDirection)) {
      return;
    }

    //loop over player positions

    possibleMovePositions.push(possiblePosition);
  });
  return possibleMovePositions;
}

function positionOutOfBoard(position: { x: number, y: number }): boolean {
  return position.x >= get(size) || position.y >= get(size) ||
    position.x < 0 || position.y < 0
}

export function cancelMove(): void {
  wallPreview.set(null);
  singlePlayerPreview.set(null);
  showPlayerPreviews();
}

export function showPlayerPreviews(): void {
  let playerPreviewsNew: any[] = [];
  getPossibleNextPawnPositons().forEach(
    (playerMove: any) => {
      playerPreviewsNew.push({
        position: playerMove,
        color: get(players)[get(currentPlayerIndex)].color,
      });
    });
  playerPreviews.set(playerPreviewsNew);
  console.log(get(playerPreviews));
}

export function noWallObstacle(playerPosition: { x: number, y: number }, moveDirection: { x: number, y: number }): boolean {
  let conflict = false;
  for (let wall of get(walls)) {
    if(wallConflictsMove(wall, playerPosition, moveDirection)){
      conflict = true;
      return conflict;
    }
  }
  return conflict;
}

function wallConflictsMove(
  wall: { position: { x: number, y: number }, isHorizontal: boolean },
  playerPosition: { x: number, y: number },
  moveDirection: {x: number, y: number}
): boolean {
  if (moveDirection.x === 0 && !wall.isHorizontal) {
    return false; // vertical wall can't hinder vertical movement in y dircetion
  }
  if (moveDirection.y === 0 && wall.isHorizontal) {
    return false; // horizontal wall can't hinder horizontal movement in x direction
  }

  //check which moveDirection it is and there if it is a conflict
  if(equalPos(moveDirection, UP)){
    if(wall.position.y === playerPosition.y - 1 && (wall.position.x === playerPosition.x -1 || wall.position.x === playerPosition.x)){
      return true;
    }
  }
  else if(equalPos(moveDirection, DOWN)){
    if(wall.position.y === playerPosition.y && (wall.position.x === playerPosition.x -1 || wall.position.x === playerPosition.x)){
      return true;
    }
  }

  else if(equalPos(moveDirection, LEFT)){
    if(wall.position.x === playerPosition.x - 1 && (wall.position.y === playerPosition.y -1 || wall.position.y === playerPosition.y)){
      return true;
    }
  }

  else if(equalPos(moveDirection, RIGTH)){
    if(wall.position.x === playerPosition.x && (wall.position.y === playerPosition.y -1 || wall.position.y === playerPosition.y)){
      return true;
    }
  }
  return false;
}

export function isWallPositionValid(
  newWall: {
    position: {
      x: number,
      y: number
    },
    isHorizontal: boolean
  }): boolean {
  if (
    newWall.position.x >= get(size) - 1 ||
    newWall.position.y >= get(size) - 1 ||
    newWall.position.x < 0 ||
    newWall.position.y < 0
  ) {
    // wall is (at least partially) outside of the board
    return false;
  }
  for (let wall of get(walls)) {
    if (isInConflict(newWall, wall)) {
      return false;
    }
  }
  return true;
}

function isInConflict(
  newWall: {
    position: {
      x: number,
      y: number
    },
    isHorizontal: boolean
  },
  wall: {
    position: {
      x: number,
      y: number
    },
    isHorizontal: boolean
  }) {
  if (equalPos(wall.position, newWall.position)) {
    //walls on same square always collide
    return true;
  }

  if (newWall.isHorizontal && wall.isHorizontal && wall.position.y === newWall.position.y) {
    //horizontal wall on same row
    const xDifference = Math.abs(wall.position.x - newWall.position.x);
    if (xDifference <= 1) return true;
  }
  if (!newWall.isHorizontal && !wall.isHorizontal && wall.position.x === newWall.position.x) {
    //vertical wall on same column
    const yDifference = Math.abs(wall.position.y - newWall.position.y);
    if (yDifference <= 1) return true;
  }
  return false;
}

function equalPos(position1: { x: number, y: number }, position2: { x: number, y: number }): boolean {
  return position1.x === position2.x && position1.y === position2.y
}

function getClickWall(clickPositionCanvas: { x: number, y: number }, canvasWidth: number): {
  position: {
    x: number,
    y: number
  },
  isHorizontal: boolean
} | null {
  for (let yBoard = 0; yBoard < get(size) - 1; yBoard++) {
    for (let xBoard = 0; xBoard < get(size) - 1; xBoard++)
      if (
        isAfterThisSquare(xBoard, clickPositionCanvas.x) &&
        isInThisSquare(yBoard, clickPositionCanvas.y)) {
        return {
          position: {
            x: xBoard,
            y: yBoard
          },
          isHorizontal: false
        }
      } else if (
        isInThisSquare(xBoard, clickPositionCanvas.x) &&
        isAfterThisSquare(yBoard, clickPositionCanvas.y)) {
        return {
          position: {
            x: xBoard,
            y: yBoard
          },
          isHorizontal: true
        }
      }
  }
  return null;
}

export function showClickedPreview(clickPositionCanvas: { x: number, y: number }, canvasWidth: number): void {
  //first test if click is a wall
  let clickedWall = getClickWall(clickPositionCanvas, canvasWidth);
  if (clickedWall) {
    if (isWallPositionValid(clickedWall)) {
      //set preview wall
      wallPreview.set(clickedWall);
      singlePlayerPreview.set(null);
      playerPreviews.set([]);
    }
    return;
    //maybe error message, that wall cannot be put on this position
  }

  //test if clickPosition is a Preview
  let clickedPawnPosition = getClickPawnPosition(clickPositionCanvas, canvasWidth);
  if (clickedPawnPosition) {
    //test if pawn is in current playerPreviews.
    if (isInPlayerPreviews(clickedPawnPosition)) {
      singlePlayerPreview.set({
        position: clickedPawnPosition,
        color: get(players)[get(currentPlayerIndex)].color,
      })
      playerPreviews.set([]);
      wallPreview.set(null);
      console.log("correct Position:", clickedPawnPosition);
    }

  }
  return;
}

function isInPlayerPreviews(position: { x: number, y: number }): boolean {
  let previews = get(playerPreviews)
  let inPlayerPreviews: boolean = false;
  for (let i = 0; i < previews.length; i++) {
    if (equalPos(previews[i].position, position)) {
      inPlayerPreviews = true;
      return true
    }
  }
  return inPlayerPreviews;
}


function getClickPawnPosition(clickPositionCanvas: { x: number, y: number }, canvasWidth: number): { x: number, y: number } | null {
  for (let yBoard = 0; yBoard < get(size); yBoard++) {
    for (let xBoard = 0; xBoard < get(size); xBoard++)
      if (
        isInThisSquare(xBoard, clickPositionCanvas.x) &&
        isInThisSquare(yBoard, clickPositionCanvas.y)) {
        return {
          x: xBoard,
          y: yBoard
        }
      }
  }
  return null;
}