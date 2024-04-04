<script lang="ts">
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import Pawn from "./Pawn.svelte";
  export let size: number = 3;

  //internal size considers space for the walls
  let internalSize = 2 * size - 1;
  let squareWidth = 4;
  let wallWidth = 1;
  let gridSize = size * squareWidth + (size - 1) * wallWidth;

  export let players: any;

  let grid = new Array(internalSize)
    .fill(0)
    .map(() => new Array(internalSize).fill(0));
</script>

<div class="grid grid-rows-{gridSize} grid-cols-{gridSize}">
  {#each grid as row, yLarge}
    {#each row as cell, xLarge}
      {#if xLarge % 2 === 0 && yLarge % 2 === 0}
        <!-- Square -->
        <div
          class="
          
          col-start-{(xLarge / 2) * (squareWidth + wallWidth) + 1} 
          row-start-{(yLarge / 2) * (squareWidth + wallWidth) + 1} 
          col-span-{squareWidth} 
          row-span-{squareWidth}"
        >
          <!-- {#each players as player}
            {#if 2 * player.position.x === xLarge && 2 * player.position.y === yLarge}
              <Pawn color={player.color} />
            {/if}
          {/each} -->
          <Square />
        </div>
      {:else if xLarge % 2 === 0}
        <!-- horizontal wall [yLarge % 2 === 1] -->
        <div
          class="
          col-start-{(xLarge / 2) * (squareWidth + wallWidth) + 1} 
          row-start-{((yLarge - 1) / 2) * (squareWidth + wallWidth) +
            squareWidth +
            1} 
          col-span-{squareWidth} 
          row-span-{wallWidth}"
        >
          <div class="w-full aspect-[1/1] bg-gray-200">
            <!-- <p>hw x: {xLarge} y: {yLarge}</p> -->
          </div>
        </div>
      {:else if yLarge % 2 === 0}
        <!-- vertical wall [xLarge % 2 === 1] -->
        <div
          class="bg-gray-1000 col-start-{((xLarge - 1) / 2) *
            (squareWidth + wallWidth) +
            squareWidth +
            1} row-start-{(yLarge / 2) * (squareWidth + wallWidth) +
            1} col-span-{wallWidth} row-span-{squareWidth}"
        >
          <div class="w-full aspect-[1/1] bg-gray-200">
            <!-- <p>vw x: {xLarge} y: {yLarge}</p> -->
          </div>
          <!-- <Wall gridHeight={squareWidth} gridLength={wallWidth}/> -->
        </div>
      {:else}
        <!-- corner wall [xLarge % 2 === 1 && yLarge % 2 === 1] -->
        <div
          class="col-start-{((xLarge - 1) / 2) * (squareWidth + wallWidth) +
            squareWidth +
            1} row-start-{((yLarge - 1) / 2) * (squareWidth + wallWidth) +
            squareWidth +
            1} col-span-{wallWidth} row-span-{wallWidth}"
        >
          <div class="w-full aspect-[1/1] bg-gray-200">
            <!-- <p>cw x: {xLarge} y: {yLarge}</p> -->
          </div>
        </div>
      {/if}
    {/each}
  {/each}
</div>
