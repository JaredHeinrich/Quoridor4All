<script lang="ts">
  import { onMount } from "svelte";
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import { setConfigurations } from "./coordinateCalculation";
  import {
    canvasClick,
    showPlayerPreviews,
  } from "./gameLogic";
  import {currentPlayerIndex, size, walls, players, playerPreviews, wallPreview} from "../../store";
  import { get } from "svelte/store";

  // export let instance: object = {
  //   revertPreview(),
  //  }; // funktioniert nicht wenn = {};

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  function handleResize(width: number) {
    setConfigurations(width, squareWidthComparedToWallWidth);
  }

  let grid = new Array($size).fill(0).map(() => new Array($size).fill(0));

  function handleClick(clickPosition: any) {
    let canvasWidth = document.getElementById("outerDiv")?.offsetWidth ?? 500; //div width or width inside of the canvas/inside configuration or last call onResize;
    let clickObject = canvasClick(
      clickPosition,
      canvasWidth
    ) ?? { isValidClick: false };

    if (!clickObject.isValidClick) {
      return;
    }

    if (clickObject.clickedWall) {
      $wallPreview = clickObject.clickedWall;

      //traditional for loop, because in foreach loop svelte does not register that the playerPreviews over all change since only the objects inside the array change
      for (let i = 0; i < $playerPreviews.length; i++) {
        $playerPreviews[i].isVisible = false;
      }
      return;
    }

    if (clickObject.clickedPawn) {
    }
  }

  
  showPlayerPreviews();

  export const revertPreview: Function = () => {
    showPlayerPreviews();
    //wallPreview = {};
  }
</script>

<div id="outerDiv">
  <Canvas onClick={handleClick} onResize={handleResize}>
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
        is_horizontal={wall.is_horizontal}
        isPreview={false}
      />
    {/each}

    {#each $playerPreviews as playerPreview, index}
      {#if playerPreview.isVisible}
        <Pawn
          xBoard={playerPreview.position.x}
          yBoard={playerPreview.position.y}
          color={playerPreview.color}
          isPreview={true}
        />
      {/if}
    {/each}
    {#if $wallPreview}
      <Wall
        xBoard={$wallPreview.position.x}
        yBoard={$wallPreview.position.y}
        isPreview={true}
        is_horizontal={$wallPreview.is_horizontal}
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
