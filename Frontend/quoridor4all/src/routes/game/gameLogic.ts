import { endOfSquare, startOfSquare, isAfterThisSquare, isInThisSquare } from "./coordinateCalculation";

export function getPossiblePlayerMoves(playerIndex: number, players: any): any[] {
  let playerPosition = players[playerIndex].position;

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

export function getPlayerPreviews(currentPlayerIndex: number, players: any[]): any[] {
  let playerPreviews: any[] = [];
  getPossiblePlayerMoves(currentPlayerIndex, players).forEach(
    (playerMove: any) => {
      console.log("player preview getPossiblePlayerMoves");
      playerPreviews.push({
        playerIndex: currentPlayerIndex,
        position: playerMove,
        isVisible: true,
      });
    });
    return playerPreviews;
}

export function checkWallObstacle(playerPosition: any, moveDirection: any, walls: any): boolean {
  for (let wall of walls) {
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

export function isWallPositionValid(newWall: any, size: number, walls: any): boolean {
  if (
    newWall.position.x >= size - 1 ||
    newWall.position.y >= size - 1 ||
    newWall.position.x < 0 ||
    newWall.position.y < 0
  ) {
    // wall is (at least partially) outside of the board
    return false;
  }
  for (let wall of walls) {
    if (equalPos(wall.position, newWall.position)) {
      //walls on same square always collide
      return false;
    }
    if (newWall.isHorizontal && wall.isHorizontal && wall.position.y === newWall.position.y) {
      //vertical wall on same row
      const xDifference = Math.abs(wall.position.x - newWall.position.x);
      if (xDifference <= 1) return false;
    }
    if (!newWall.isHorizontal && !wall.isHorizontal && wall.position.x === newWall.position.x) {
      //horizontal wall on same column
      const yDifference = Math.abs(wall.position.y - newWall.position.y);
      if (yDifference <= 1) return false;
    }
  }
  return true;
}

function equalPos(position1: any, position2: any) {
  return position1.x === position2.x && position1.y === position2.y
}

export function canvasClick(clickPositionCanvas: any, canvasWidth: number, size: number, walls: any, players: any) {
  let clickedWall = isClickWall(clickPositionCanvas, canvasWidth, size);
  if (clickedWall) {
    if (isWallPositionValid(clickedWall, size, walls)) {
      //set preview wall
      return{
        isValidClick: true,
        clickedWall: clickedWall
      };
    }
    return { isValidClick: false};
    //maybe error message, that wall cannot be put on this position
  }

  let clickedPawn = isClickPawn(clickPositionCanvas, canvasWidth, size);
  if(clickedPawn){
    //maybe additional check if clicked position is a possible pawn move
    return {
      isValidClick: true,
      clickedPawn: clickedPawn
    };
  }
  return{ isValidClick: false}
}

function isClickWall(clickPositionCanvas: any, canvasWidth: number, size: number): any {
  for (let yBoard = 0; yBoard < size - 1; yBoard++) {
    for (let xBoard = 0; xBoard < size - 1; xBoard++)
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
  return false;
}

function isClickPawn(clickPositionCanvas: any, canvasWidth: number, size: number): any {
  for (let yBoard = 0; yBoard < size - 1; yBoard++) {
    for (let xBoard = 0; xBoard < size - 1; xBoard++)
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