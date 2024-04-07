<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import { startOfSquare, centerOfSquare, endOfSquare, setConfigurations } from './coordinateCalculation';

  export let size: number = 9;

  let divWidth2: number;
  $: console.log(divWidth2);
  let divWidth = 980;
  const canvasWidth = divWidth || 1000;

  //internal size considers space for the walls
  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls
  setConfigurations(size, canvasWidth, squareWidthComparedToWallWidth);

  export let players: any;

  export let walls: any;

  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));
</script>

<!-- <Board players={[]} /> -->
<div id="outerDiv" bind:offsetWidth={divWidth2}>
  <Canvas width={canvasWidth}>
    <!-- Grid -->
    {#each grid as row, yBoard}
      {#each row as cell, xBoard}
        <Square
          xBoard={xBoard}
          yBoard={yBoard}
        />
      {/each}
    {/each}

    {#each players as player, index}
      <Pawn
        xBoard={player.position.x}
        yBoard={player.position.y}
        color={player.color}
      />
    {/each}

    <!--{#each walls as wall, index}
      {#if wall.isVertical}
        <Wall
          topLeftCornerX={startOfSquare(wall.position.x)}
          topLeftCornerY={endOfSquare(wall.position.y)}
          height={wallWidthCanvas}
          width={2*squareWidthCanvas + wallWidthCanvas}
          isPreview={false}
        />
      {:else}
        <Wall
          topLeftCornerX={endOfSquare(wall.position.x)}
          topLeftCornerY={startOfSquare(wall.position.y)}
          height={2*squareWidthCanvas + wallWidthCanvas}
          width={wallWidthCanvas}
          isPreview={false}
        />
      {/if}
    {/each}
    <Wall
          topLeftCornerX={endOfSquare(7)}
          topLeftCornerY={startOfSquare(7)}
          height={2*squareWidthCanvas + wallWidthCanvas}
          width={wallWidthCanvas}
          isPreview={true}
        /> -->
  </Canvas>
</div>
