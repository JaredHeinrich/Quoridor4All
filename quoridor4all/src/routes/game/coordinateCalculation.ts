import { size} from "../../store";
import { get } from 'svelte/store';

let canvasWidth: number;
let squareWidthComparedToWallWidth: number;
export let wallWidthCanvas: number;
export let squareWidthCanvas: number;
  
// general configurations
export function setConfigurations(_canvasWidth: number, _squareWidthComparedToWallWidth: number) {
  canvasWidth =  _canvasWidth;
  squareWidthComparedToWallWidth = _squareWidthComparedToWallWidth;

  let numberWallWidthsInCanvas = squareWidthComparedToWallWidth +( get(size) - 1) * (squareWidthComparedToWallWidth + 1);
  wallWidthCanvas = canvasWidth / numberWallWidthsInCanvas;
  squareWidthCanvas = wallWidthCanvas * squareWidthComparedToWallWidth;
}

export function startOfSquare(boardCoordinate: number): number {
  return boardCoordinate * (wallWidthCanvas + squareWidthCanvas);
}

export function centerOfSquare(boardCoordinate: number): number {
  return (
    boardCoordinate * (wallWidthCanvas + squareWidthCanvas) +
    squareWidthCanvas / 2
  );
}

export function endOfSquare(boardCoordinate: number): number {
  return (
    (boardCoordinate + 1) * (squareWidthCanvas + wallWidthCanvas) -
    wallWidthCanvas
  );
}


//canvas to board
export function isAfterThisSquare(boardCoordinate: number, canvasCoordinate: number): boolean {
  return endOfSquare(boardCoordinate) <= canvasCoordinate && canvasCoordinate <= startOfSquare(boardCoordinate + 1)
}

export function isInThisSquare(boardCoordinate: number, canvasCoordinate: number): boolean {
  return startOfSquare(boardCoordinate) <= canvasCoordinate && canvasCoordinate <= endOfSquare(boardCoordinate);
}