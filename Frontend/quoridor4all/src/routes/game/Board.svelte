<script lang="ts">
  import { onDestroy, onMount } from "svelte";
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
  import {
    getPossiblePlayerMoves,
    checkWallObstacle,
    isWallPositionValid,
  } from "./gameLogic";

  export let size: number = 9;
  export let players: any;
  export let walls: any;
  export let currentPlayerIndex: number;

  let canvasWidth: number = 500;

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  setConfigurations(size, canvasWidth, squareWidthComparedToWallWidth);
  console.log("canvas Width", canvasWidth);

  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));

  let previewPlayers: any = [];
  let wallPreview: any = {
    isVertical: false,
    position: {
      x: -1,
      y: -1,
    },
    isVisible: true,
  };

  // console.log("Wall test", isWallPositionValid({ wallPreview }));
  function handleClick(clickPosition: any) {
    console.log("handleClick");
  }

  getPossiblePlayerMoves(currentPlayerIndex, players).forEach(
    (playerMove: any) => {
      previewPlayers.push({
        playerIndex: currentPlayerIndex,
        position: playerMove,
      });
    }
  );
</script>

<div>
  <Canvas width={canvasWidth} onClick={handleClick}>
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
      <Pawn
        xBoard={previewPlayer.position.x}
        yBoard={previewPlayer.position.y}
        color={players[previewPlayer.playerIndex].color}
        isPreview={true}
      />
    {/each}
    <!-- {#if wallPreview.isVisible}
      <Wall
        xBoard={wallPreview.position.x}
        yBoard={wallPreview.position.y}
        isPreview={true}
        isVertical={wallPreview.isVertical}
      />
    {/if} -->
  </Canvas>
</div>

<style>
  /* Damit der div-Container immer die volle Breite des umschließenden Elements einnimmt */
  div {
    width: 100%;
  }
</style>
