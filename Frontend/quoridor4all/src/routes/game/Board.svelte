<script lang="ts">
  import { Button } from "flowbite-svelte";
  import Square from "./Square.svelte";
  export let size = 9;

  //internal size considers space for the walls
  let internalSize = 2 * size - 1;
  let squareWidth = 4;
  let wallWidth = 1;
  let gridSize = size * squareWidth + wallWidth * (size - 1);

  let grid = new Array(internalSize)
    .fill(0)
    .map(() => new Array(internalSize).fill(0));
</script>

<!-- <div class="w-full aspect-[1/1] bg-gray-500"> -->
<!-- maybe  -->
<!-- <div class="m-4 bg-gray-100 w-full aspect-[1/1]">
    {#each Array(internalSize) as _, xPosition}
      
       <Square /> -->
<!-- {/each} -->

<!-- </div> -->

<!-- <div class="flex flex-wrap" style={`width: ${size * 1.5}rem`}>
    {#each Array(internalSize) as _, xPosition}
      {#each Array(internalSize) as _2, yPosition}
        <Square/>
      {/each}
    {/each}
  </div> -->
<!-- </div> -->

<div class="grid grid-cols-{gridSize} grid-rows-{gridSize} ">
  {#each grid as row, yLarge}
    {#each row as cell, xLarge}
      {#if xLarge % 2 === 0 && yLarge % 2 === 0}
        <!-- Square -->
        <div
          class="
          col-start-{(xLarge / 2) * (squareWidth + wallWidth) + 1} 
          row-start-{(yLarge / 2) *(squareWidth + wallWidth) + 1} 
          col-span-{squareWidth} 
          row-span-{squareWidth}"
        >
          <Square />
          <!-- <p>square x: {xLarge} y: {yLarge}</p> -->
        </div>
      {:else if xLarge % 2 === 0}
        <!-- horizontal wall [yLarge % 2 === 1] -->
        <div
          class="
          col-start-{(xLarge / 2) * (squareWidth + wallWidth) + 1} 
          row-start-{((yLarge - 1) / 2) * (squareWidth + wallWidth) + squareWidth + 1} 
          col-span-{squareWidth} 
          row-span-{wallWidth}"
        >
          <!-- <p>hw x: {xLarge} y: {yLarge}</p> -->
          <p>hw</p>
        </div>
      {:else if yLarge % 2 === 0}
        <!-- vertical wall [xLarge % 2 === 1] -->
        <div
          class="col-start-{((xLarge - 1) / 2) * (squareWidth + wallWidth) +
            squareWidth + 1} row-start-{(yLarge / 2) * (squareWidth + wallWidth) +1 } col-span-{wallWidth} row-span-{squareWidth}"
        >
          <!-- <Button>Wand</Button> -->
          <p>vw</p>
          <!-- <p>vw x: {xLarge} y: {yLarge}</p> -->

        </div>
      {:else}
        <!-- corner wall [xLarge % 2 === 1 && yLarge % 2 === 1] -->
        <div
          class="col-start-{((xLarge - 1) / 2) * (squareWidth + wallWidth) +
            squareWidth + 1} row-start-{((yLarge - 1) / 2) *
            (squareWidth + wallWidth) +
            squareWidth + 1} col-span-{wallWidth} row-span-{wallWidth}"
        >
          <!-- <Button>Wand</Button> -->
          <!-- <p>cw x: {xLarge} y: {yLarge}</p> -->
          <p>cw</p>
        </div>
      {/if}
    {/each}
  {/each}
</div>
