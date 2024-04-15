import { isAfterThisSquare, isInThisSquare } from "./coordinateCalculation";
import { players, currentPlayerIndex, size, walls, playerPreviews } from "../../store"
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

export function showPlayerPreviews() {
  let playerPreviewsNew: any[] = [];
  getPossiblePlayerMoves().forEach(
    (playerMove: any) => {
      console.log("player preview getPossiblePlayerMoves");
      playerPreviewsNew.push({
        position: playerMove,
        color: "red",
        isVisible: true,
      });
    });
  console.log("Player Previews New: ", playerPreviewsNew);
  playerPreviews.set(playerPreviewsNew);
  console.log("PlayerPreviews", get(playerPreviews));
}

export function checkWallObstacle(playerPosition: any, moveDirection: any): boolean {
  for (let wall of get(walls)) {
    if (moveDirection.x === 0 && !wall.is_horizontal) {
      return false; // vertical wall can't hinder vertical movement in y dircetion
    }
    if (moveDirection.y === 0 && wall.is_horizontal) {
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

export function isWallPositionValid(newWall: any): boolean {
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
    if (isInConflictWith(newWall, wall)) {
      return false;
    }
  }
  return true;
}

function isInConflictWith(newWall: any, wall: any) {
  if (equalPos(wall.position, newWall.position)) {
    //walls on same square always collide
    return true;
  }

  if (newWall.is_horizontal && wall.is_horizontal && wall.position.y === newWall.position.y) {
    //horizontal wall on same row
    const xDifference = Math.abs(wall.position.x - newWall.position.x);
    if (xDifference <= 1) return true;
  }
  if (!newWall.is_horizontal && !wall.is_horizontal && wall.position.x === newWall.position.x) {
    //vertical wall on same column
    const yDifference = Math.abs(wall.position.y - newWall.position.y);
    if (yDifference <= 1) return true;
  }
  return false;
}

function equalPos(position1: any, position2: any) {
  return position1.x === position2.x && position1.y === position2.y
}

export function canvasClick(clickPositionCanvas: any, canvasWidth: number) {
  let clickedWall = isClickWall(clickPositionCanvas, canvasWidth);
  if (clickedWall) {
    if (isWallPositionValid(clickedWall)) {
      //set preview wall
      return {
        isValidClick: true,
        clickedWall: clickedWall
      };
    }
    return { isValidClick: false };
    //maybe error message, that wall cannot be put on this position
  }

  let clickedPawn = isClickPawn(clickPositionCanvas, canvasWidth);
  if (clickedPawn) {
    //maybe additional check if clicked position is a possible pawn move
    return {
      isValidClick: true,
      clickedPawn: clickedPawn
    };
  }
  return { isValidClick: false }
}

function isClickWall(clickPositionCanvas: any, canvasWidth: number): any {
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
          is_horizontal: false
        }
      } else if (
        isInThisSquare(xBoard, clickPositionCanvas.x) &&
        isAfterThisSquare(yBoard, clickPositionCanvas.y)) {
        return {
          position: {
            x: xBoard,
            y: yBoard
          },
          is_horizontal: true
        }
      }
  }
  return false;
}

function isClickPawn(clickPositionCanvas: any, canvasWidth: number): any {
  for (let yBoard = 0; yBoard < get(size) - 1; yBoard++) {
    for (let xBoard = 0; xBoard < get(size) - 1; xBoard++)
      if (
        isInThisSquare(xBoard, clickPositionCanvas.x) &&
        isInThisSquare(yBoard, clickPositionCanvas.y)) {
        return {
          position: {
            x: xBoard,
            y: yBoard
          },
        }
      }
  }
}