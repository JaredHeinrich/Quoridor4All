import { isAfterThisSquare, isInThisSquare } from "./coordinateCalculation";
import { players, currentPlayerIndex, size, walls, playerPreviews, wallPreview, singlePlayerPreview } from "../../store"
import { get } from 'svelte/store';

function getPossiblePlayerMoves(): any[] {
  let playerPosition = get(players)[get(currentPlayerIndex)].position;

  //all surrounding positions are possible moveDirections at first
  let possibleMoveDirections = [
    { x: +1, y: 0 }, //right
    { x: 0, y: +1 }, //down
    { x: -1, y: 0 }, //left
    { x: 0, y: -1 }, //up
  ];
  let possibleMovePositions: any = [];

  possibleMoveDirections.forEach((possibleMoveDirection) => {
    let possiblePosition = {
      x: playerPosition.x + possibleMoveDirection.x,
      y: playerPosition.y + possibleMoveDirection.y,
    };
    // if (checkWallObstacle(possiblePosition, possibleMoveDirection)) {
    //   return;
    // }

    //loop over player positions

    possibleMovePositions.push(possiblePosition);
  });
  return possibleMovePositions;
}

export function cancelMove(): void {
  wallPreview.set(null);
  singlePlayerPreview.set(null);
  showPlayerPreviews();
}

export function showPlayerPreviews(): void {
  let playerPreviewsNew: any[] = [];
  getPossiblePlayerMoves().forEach(
    (playerMove: any) => {
      playerPreviewsNew.push({
        position: playerMove,
        color: "red",
      });
    });
  playerPreviews.set(playerPreviewsNew);
}

export function checkWallObstacle(playerPosition: { x: number, y: number }, moveDirection: { x: number, y: number }): boolean {
  for (let wall of get(walls)) {
    if (moveDirection.x === 0 && !wall.isHorizontal) {
      return false; // vertical wall can't hinder vertical movement in y dircetion
    }
    if (moveDirection.y === 0 && wall.isHorizontal) {
      return false; // horizontal wall can't hinder horizontal movement in x direction
    }

    let possiblePosition = {
      x: playerPosition.x + moveDirection.x,
      y: playerPosition.y + moveDirection.y,
    };

    if ((wall.position = possiblePosition)) {
      return false; // wall
    }
  }
  return true;

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