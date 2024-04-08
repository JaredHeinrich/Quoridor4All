<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import {
    startOfSquare,
    centerOfSquare,
    endOfSquare,
    setConfigurations,
  } from "./coordinateCalculation";

  export let size: number = 9;
  export let players: any;
  export let walls: any;
  export let currentPlayerIndex: number;

  let divWidth = 500;
  const canvasWidth = divWidth || 1000;

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls
  setConfigurations(size, canvasWidth, squareWidthComparedToWallWidth);


  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));

  let previewPlayers: any = [];
  getPossiblePlayerMoves(currentPlayerIndex).forEach(
    (playerMove: any) => {
      previewPlayers.push({
        playerIndex: currentPlayerIndex,
        position: playerMove,
      });
    }
  );

  function getPossiblePlayerMoves(playerIndex: number): any {
    let playerPosition = players[0].position;

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

  function checkWallObstacle(playerPosition: any, moveDirection: any): boolean {
    walls.forEach((wall: any) => {
      if (moveDirection.x === 0 && wall.isVertical) {
        return false; // vertical wall can't hinder vertical movement in y dircetion
      }
      if (moveDirection.y === 0 && !wall.isVertical) {
        return false; // horizontal wall can't hinder horizontal movement in x direction
      }

      let possiblePosition = {
        x: playerPosition.x + moveDirection.x,
        y: playerPosition.y + moveDirection.y,
      };

      if ((wall.position = possiblePosition)) {
        return false; // wall
      }
    });
    return true;
  }
</script>

<!-- <Board players={[]} /> -->
<div id="outerDiv" bind:offsetWidth={divWidth}>
  <Canvas width={canvasWidth}>
    <!-- Grid -->
    {#each grid as row, yBoard}
      {#each row as cell, xBoard}
        <Square {xBoard} {yBoard} />
      {/each}
    {/each}

    {#each players as player, index}
      <Pawn
        xBoard={player.position.x}
        yBoard={player.position.y}
        color={player.color}
        isPreview={false}
      />
    {/each}

    {#each walls as wall, index}
      <Wall
        xBoard={wall.position.x}
        yBoard={wall.position.y}
        isVertical={wall.isVertical}
        isPreview={false}
      />
    {/each}

    {#each previewPlayers as previewPlayer, index}
      <Pawn xBoard={previewPlayer.position.x} yBoard={previewPlayer.position.y} color={players[previewPlayer.playerIndex].color} isPreview={true}/>
    {/each}
  </Canvas>
</div>
