<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";

  export let size: number = 9;

  let divWidth2: number;
  $: console.log(divWidth2)
  let divWidth = 980;
  const canvasWidth = divWidth || 1000;
  

  //internal size considers space for the walls
  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls
  let numberWallWidthsInCanvas =
    squareWidthComparedToWallWidth +
    (size - 1) * (squareWidthComparedToWallWidth + 1);
  let wallWidthCanvas = canvasWidth / numberWallWidthsInCanvas;
  let squareWidthCanvas = wallWidthCanvas * squareWidthComparedToWallWidth;

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
          topLeftCornerX={xBoard * (wallWidthCanvas + squareWidthCanvas)}
          topLeftCornerY={yBoard * (wallWidthCanvas + squareWidthCanvas)}
          width={squareWidthCanvas}
        />
      {/each}
    {/each}

    {#each players as player, index}
      <Pawn centerX ={player.position.x * (wallWidthCanvas + squareWidthCanvas) + squareWidthCanvas/2 } centerY={player.position.y * (wallWidthCanvas + squareWidthCanvas) + squareWidthCanvas/2} radius={squareWidthCanvas/2 * 0.8} color={player.color}/>
    {/each}
  </Canvas>
</div>
