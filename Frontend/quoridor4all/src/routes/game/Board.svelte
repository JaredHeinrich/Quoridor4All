<script lang="ts">
  import { onDestroy, onMount, tick } from "svelte";
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
    canvasClick
  } from "./gameLogic";

  export let size: number = 9;
  export let players: any;
  export let walls: any;
  export let currentPlayerIndex: number;

  let canvasWidth: number = 500;

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  setConfigurations(size, canvasWidth, squareWidthComparedToWallWidth);
  console.log("canvas Width", canvasWidth);

  let divWidth: number;
  

  onMount(async () => {
    // Warte auf die nächste DOM-Aktualisierung
    await tick();
    console.log("div width", divWidth);
    canvasWidth = divWidth;
  });
  

  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));

  let previewPlayers: any = [];
  let wallPreview: any = {
    isHorizontal: true,
    position: {
      x: 0,
      y: 0,
    },
    isVisible: false,
  };

  console.log("Wall test", isWallPositionValid(wallPreview, size, walls));

  function handleClick(clickPosition: any) {
    canvasClick(clickPosition, canvasWidth, size, walls, players);
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

<div bind:offsetWidth={divWidth}>
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
        isHorizontal={wall.isHorizontal}
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
    {#if wallPreview.isVisible}
      <Wall
        xBoard={wallPreview.position.x}
        yBoard={wallPreview.position.y}
        isPreview={true}
        isHorizontal={wallPreview.isHorizontal}
      />
    {/if}
  </Canvas>
</div>

<style>
  div {
    width: 100%;
  }
</style>
