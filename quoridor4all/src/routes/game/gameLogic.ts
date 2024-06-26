import { isAfterThisSquare, isInThisSquare } from "./coordinateCalculation";
import { players, currentPlayerIndex, size, walls, playerPreviews, wallPreview, singlePlayerPreview, gameRunning, winnerName } from "../../store"
import { get } from 'svelte/store';
import { invoke } from "@tauri-apps/api/tauri";
import { wallToRust } from "./typeConverter"
import { toWin } from "../navigation";

// const UP: { x: number, y: number } = { x: 0, y: -1 };
// const DOWN: { x: number, y: number } = { x: 0, y: +1 };
// const LEFT: { x: number, y: number } = { x: -1, y: 0 };
// const RIGTH: { x: number, y: number } = { x: +1, y: 0 };


export async function doTurn(): Promise<void> {
  const currentIndex = get(currentPlayerIndex);
  const pickedPawnPreview = get(singlePlayerPreview);
  const pickedWallPreview = get(wallPreview);

  if (pickedPawnPreview) {
    const position: { x: number, y: number } = pickedPawnPreview.position;
    const result: any = await invoke("move_pawn", { newPosition: position });
    const newPosition: { x: number, y: number } = result[0];
    
    const won: boolean = result[1];

    if(won){
      winnerName.set(get(players)[currentIndex].playerName);
      toWin();
      return;
    }

    players.update((players) => {
      players[currentIndex].position = newPosition; // Update Position of the player
      return players;
    });

    nextPlayer();
  }
  //check if there is a picked wall and if current player is allowed to set the wall
  else if (pickedWallPreview && get(players)[currentIndex].wallQuantity > 0) {
    const wall = wallToRust(pickedWallPreview); //type conversion
    await invoke("place_wall", { wall: wall });
    //set the new wall
    walls.update((existingWalls) => {
      existingWalls.push(pickedWallPreview || { position: { x: 0, y: 0 }, isHorizontal: true }); //should not happen, just for typesafety
      return existingWalls;
    })

    //reduce the wall quantity of the current Player
    players.update((players) => {
      players[currentIndex].wallQuantity = players[currentIndex].wallQuantity - 1;
      return players;
    });
    nextPlayer();
  }

  await showPlayerPreviews();
}

export async function undoLastTurn(): Promise<void> {
  const result: any = await invoke("undo_last_move");

  const currentIndex = get(currentPlayerIndex);
  const lastIndex = (currentIndex == 0) ? (3) : (currentIndex - 1);

  const isPlayerMove = result[1];

  //show backend result in frontend
  if (isPlayerMove) {  //player move
    const newPosition: { x: number, y: number } = result[0];
    

    players.update((players) => {
      players[lastIndex].position = newPosition;
      return players;
    });

    previousPlayer();

  } else { // wall move

    walls.update((walls) => {
      walls.pop();
      return walls;
    })

    players.update((players) => {
      players[lastIndex].wallQuantity += 1;
      return players;
    });

    previousPlayer();

  }

  await showPlayerPreviews();

}

export async function cancelMove(): Promise<void> {
  await showPlayerPreviews();
}

async function showPlayerPreviews(): Promise<void> {
  //other previews must not be showns
  wallPreview.set(null);
  singlePlayerPreview.set(null);

  //reset player previews first to not have two times the same playerPreviews shown
  playerPreviews.set([]);

  const playerPreviewsNew: { position: { x: number, y: number }, color: string, }[] = [];
  //get all possible next pawn moves and add each one to playerPreviews
  const possibleMoves = await getPossibleMovesBackend();
  possibleMoves.forEach(
    (position) => {
      playerPreviewsNew.push({
        position,
        color: get(players)[get(currentPlayerIndex)].color,
      });
    });
  playerPreviews.set(playerPreviewsNew);
}

export async function showClickedPreview(clickPositionCanvas: { x: number, y: number }, canvasWidth: number): Promise<void> {
  //first test if click is a wall
  const clickedWall = getClickWall(clickPositionCanvas, canvasWidth);
  if (clickedWall) {
    if (await checkWallBackend(clickedWall)) {
      //set preview wall and reset player previews
      wallPreview.set(clickedWall);
      singlePlayerPreview.set(null);
      playerPreviews.set([]);
    }
    //definitely wall click -->can finish here
    return;
  }

  //test if clickPosition is a Preview
  const clickedPawnPosition = getClickPawnPosition(clickPositionCanvas, canvasWidth);
  if (clickedPawnPosition) {
    //test if pawn is in current playerPreviews.
    if (isInPlayerPreviews(clickedPawnPosition)) {
      //reset single player preview first to not show it twice
      singlePlayerPreview.set(null);

      // only show one player preview (singlePlayerPreview)
      singlePlayerPreview.set({
        position: clickedPawnPosition,
        color: get(players)[get(currentPlayerIndex)].color,
      })
      //reset other previews
      playerPreviews.set([]);
      wallPreview.set(null);
    }
  }
  return;
}

function nextPlayer() {
  currentPlayerIndex.set((get(currentPlayerIndex) + 1) % 4);
}

function previousPlayer() {
  const currentIndex = get(currentPlayerIndex);
  if (currentIndex == 0) {
    currentPlayerIndex.set(3)
  } else {
    currentPlayerIndex.set(currentIndex - 1)
  }
}

async function getPossibleMovesBackend(): Promise<{ x: number, y: number }[]> {
  return await invoke("get_possible_moves");
}

async function checkWallBackend(newWall: {
  position: {
    x: number,
    y: number
  },
  isHorizontal: boolean
}): Promise<boolean> {
  const wall = wallToRust(newWall);
  return await invoke("check_wall", { wall: wall });
}

function equalPos(position1: { x: number, y: number }, position2: { x: number, y: number }): boolean {
  return position1.x === position2.x && position1.y === position2.y
}

//check if a click on a canvas was on a wall, returns clicked wall or null
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

function isInPlayerPreviews(position: { x: number, y: number }): boolean {
  for (let preview of get(playerPreviews)) {
    if (equalPos(preview.position, position)) {
      return true
    }
  }
  return false;
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



export function initializeGame() {
  players.set([
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
      playerName: "Spieler 2",
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

  walls.set([]);
  currentPlayerIndex.set(0);

  playerPreviews.set([]);
  singlePlayerPreview.set(null);
  wallPreview.set(null);
  gameRunning.set(false);
}


// function getPossibleNextPawnPositons(): { x: number, y: number }[] {
//   let playerPosition = get(players)[get(currentPlayerIndex)].position;

//   //all surrounding positions are possible moveDirections at first
//   let possibleMoveDirections = [UP, DOWN, LEFT, RIGTH];
//   let possibleMovePositions: { x: number, y: number }[] = [];

//   possibleMoveDirections.forEach((possibleMoveDirection) => {
//     let possibleNewPositions: { x: number, y: number }[] = getPossiblePositionsForDirection(possibleMoveDirection, playerPosition)

//     //append found positions to all Positions
//     possibleMovePositions = possibleMovePositions.concat(possibleNewPositions);
//   });
//   return possibleMovePositions;
// }

// function getPossiblePositionsForDirection(
//   moveDirection: { x: number, y: number },
//   playerPosition: { x: number, y: number },
// ): { x: number, y: number }[] {

//   let possiblePosition = {
//     x: playerPosition.x + moveDirection.x,
//     y: playerPosition.y + moveDirection.y,
//   };

//   if (positionOutOfBoard(possiblePosition)) {
//     return [];
//   }

//   if (isWallObstacle(playerPosition, moveDirection)) {
//     return [];
//   }

//   //check if possiblePosition is already taken by another pawn
//   if (isPositionTaken(possiblePosition)) {

//     //if blocking player is infront of a wall
//     if (isWallObstacle(possiblePosition, moveDirection)) {

//       if (moveDirection.x === 0) //up or down
//       {
//         //player can move left or right from blocking player
//         return getPossiblePositionsForDirection(LEFT, possiblePosition)
//           .concat(getPossiblePositionsForDirection(RIGTH, possiblePosition))
//       }
//       else if (moveDirection.y === 0) //left or right
//       {
//         //player can move up or down from blocking player
//         return getPossiblePositionsForDirection(UP, possiblePosition)
//           .concat(getPossiblePositionsForDirection(DOWN, possiblePosition))
//       } else {
//         throw Error("Direction must have one coordinate === 0")
//       }
//     }
//     else {
//       //no wall obstacle behind player --> jump over and try behind player
//       return getPossiblePositionsForDirection(possiblePosition, moveDirection);
//     }
//   }

//   //position is valid, as there is no reason for not valid position
//   return [possiblePosition];
// }

// function isPositionTaken(position: { x: number, y: number }): boolean {
//   for (let player of get(players)) {
//     if (equalPos(player.position, position)) {
//       return true;
//     }
//   }
//   return false;
// }

// function positionOutOfBoard(position: { x: number, y: number }): boolean {
//   return position.x >= get(size) || position.y >= get(size) ||
//     position.x < 0 || position.y < 0
// }

// function isWallObstacle(playerPosition: { x: number, y: number }, moveDirection: { x: number, y: number }): boolean {
//   for (let wall of get(walls)) {
//     if (wallConflictsMove(wall, playerPosition, moveDirection)) {
//       return true;
//     }
//   }
//   return false;
// }

// function wallConflictsMove(
//   wall: { position: { x: number, y: number }, isHorizontal: boolean },
//   playerPosition: { x: number, y: number },
//   moveDirection: { x: number, y: number }
// ): boolean {
//   if (moveDirection.x === 0 && !wall.isHorizontal) {
//     return false; // vertical wall can't hinder vertical movement in y dircetion
//   }
//   if (moveDirection.y === 0 && wall.isHorizontal) {
//     return false; // horizontal wall can't hinder horizontal movement in x direction
//   }

//   //check which moveDirection it is and there if it is a conflict
//   if (equalPos(moveDirection, UP)) {
//     if (wall.position.y === playerPosition.y - 1 && (wall.position.x === playerPosition.x - 1 || wall.position.x === playerPosition.x)) {
//       return true;
//     }
//   }
//   else if (equalPos(moveDirection, DOWN)) {
//     if (wall.position.y === playerPosition.y && (wall.position.x === playerPosition.x - 1 || wall.position.x === playerPosition.x)) {
//       return true;
//     }
//   }

//   else if (equalPos(moveDirection, LEFT)) {
//     if (wall.position.x === playerPosition.x - 1 && (wall.position.y === playerPosition.y - 1 || wall.position.y === playerPosition.y)) {
//       return true;
//     }
//   }

//   else if (equalPos(moveDirection, RIGTH)) {
//     if (wall.position.x === playerPosition.x && (wall.position.y === playerPosition.y - 1 || wall.position.y === playerPosition.y)) {
//       return true;
//     }
//   }
//   return false;
// }

// function isWallPositionValid(
//   newWall: {
//     position: {
//       x: number,
//       y: number
//     },
//     isHorizontal: boolean
//   }): boolean {
//   if (
//     newWall.position.x >= get(size) - 1 ||
//     newWall.position.y >= get(size) - 1 ||
//     newWall.position.x < 0 ||
//     newWall.position.y < 0
//   ) {
//     // wall is (at least partially) outside of the board
//     return false;
//   }
//   //check if it conflicts with any existing wall
//   for (let wall of get(walls)) {
//     if (isInConflict(newWall, wall)) {
//       return false;
//     }
//   }
//   return true;
// }

// function isInConflict(
//   newWall: {
//     position: {
//       x: number,
//       y: number
//     },
//     isHorizontal: boolean
//   },
//   wall: {
//     position: {
//       x: number,
//       y: number
//     },
//     isHorizontal: boolean
//   }): boolean {
//   if (equalPos(wall.position, newWall.position)) {
//     //walls on same square always collide
//     return true;
//   }

//   if (newWall.isHorizontal && wall.isHorizontal && wall.position.y === newWall.position.y) {
//     //horizontal wall on same row --> if x difference is <=1: collision
//     const xDifference = Math.abs(wall.position.x - newWall.position.x);
//     return xDifference <= 1;
//   }
//   if (!newWall.isHorizontal && !wall.isHorizontal && wall.position.x === newWall.position.x) {
//     //vertical wall on same column --> if y difference is <=1: collision
//     const yDifference = Math.abs(wall.position.y - newWall.position.y);
//     return yDifference <= 1;
//   }
//   return false;
// }
