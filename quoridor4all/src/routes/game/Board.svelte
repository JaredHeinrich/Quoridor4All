<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import { setConfigurations } from "./coordinateCalculation";
  import { showPlayerPreviews } from "./gameLogic";
  import {
    size,
    walls,
    players,
    playerPreviews,
    wallPreview,
    singlePlayerPreview,
  } from "../../store";
  import { onMount } from "svelte";

  const squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  function handleResize(width: number) {
    setConfigurations(width, squareWidthComparedToWallWidth);
  }

  let grid = new Array($size).fill(0).map(() => new Array($size).fill(0));

  onMount(()=>{
    showPlayerPreviews();
  });
</script>

<div>
  <Canvas onResize={handleResize}>
    <!-- Grid -->
    {#each grid as row, yBoard}
      {#each row as cell, xBoard}
        <Square {xBoard} {yBoard} />
      {/each}
    {/each}

    {#each $players as player, index}
      <Pawn
        xBoard={player.position.x}
        yBoard={player.position.y}
        color={player.color}
        isPreview={false}
      />
    {/each}

    {#each $walls as wall, index}
      <Wall
        xBoard={wall.position.x}
        yBoard={wall.position.y}
        isHorizontal={wall.isHorizontal}
        isPreview={false}
      />
    {/each}

    {#each $playerPreviews as playerPreview, index}
      <Pawn
        xBoard={playerPreview.position.x}
        yBoard={playerPreview.position.y}
        color={playerPreview.color}
        isPreview={true}
      />
    {/each}

    {#if $singlePlayerPreview}
      <Pawn
        xBoard={$singlePlayerPreview.position.x}
        yBoard={$singlePlayerPreview.position.y}
        color={$singlePlayerPreview.color}
        isPreview={false}
      />
    {/if}
    
    {#if $wallPreview}
      <Wall
        xBoard={$wallPreview.position.x}
        yBoard={$wallPreview.position.y}
        isPreview={true}
        isHorizontal={$wallPreview.isHorizontal}
      />
    {/if}
  </Canvas>
</div>

<style>
  div {
    width: 100%;
    height: 100%;
  }
</style>
