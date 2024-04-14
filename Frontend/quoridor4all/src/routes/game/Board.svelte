<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import { setConfigurations } from "./coordinateCalculation";
  import { getPossiblePlayerMoves, canvasClick } from "./gameLogic";

  export let size: number = 9;
  export let players: any[] = [];
  export let walls: any[] = [];
  export let currentPlayerIndex: number;

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  function handleResize(width: number) {
    setConfigurations(size, width, squareWidthComparedToWallWidth);
  }

  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));

  let playerPreviews: any[] = [];
  let wallPreview: any = {
    wall: {
      isHorizontal: true,
      position: {
        x: 3,
        y: 4,
      },
    },
    isVisible: true,
  };

  function handleClick(clickPosition: any) {
    let canvasWidth = document.getElementById("outerDiv")?.offsetWidth ?? 500; //div width or width inside of the canvas/inside configuration or last call onResize;
    let clickObject = canvasClick(
      clickPosition,
      canvasWidth,
      size,
      walls,
      players
    ) ?? { isValidClick: false };

    if (!clickObject.isValidClick) {
      return;
    }

    if (clickObject.clickedWall) {
      wallPreview = {
        wall: clickObject.clickedWall,
        isVisible: true,
      };
      console.log("wallPreview", wallPreview);

      //traditional for loop, because in foreach loop svelte does not register that the playerPreviews over all change since only the objects inside the array change
      for(let i = 0; i < playerPreviews.length; i++){
        playerPreviews[i].isVisible = false;
      }
      console.log(playerPreviews);
      return;
    }

    if (clickObject.clickedPawn) {
    }
  }

  getPossiblePlayerMoves(currentPlayerIndex, players).forEach(
    (playerMove: any) => {
      console.log("player preview getPossiblePlayerMoves")
      playerPreviews.push({
        playerIndex: currentPlayerIndex,
        position: playerMove,
        isVisible: true,
      });
    }
  );
</script>

<div id="outerDiv">
  <Canvas onClick={handleClick} onResize={handleResize}>
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

    {#each playerPreviews as playerPreview, index}
      {#if playerPreview.isVisible}
        <Pawn
          xBoard={playerPreview.position.x}
          yBoard={playerPreview.position.y}
          color={players[playerPreview.playerIndex].color}
          isPreview={true}
        />
      {/if}
    {/each}
    {#if wallPreview.isVisible}
    <Wall
      xBoard={wallPreview.wall.position.x}
      yBoard={wallPreview.wall.position.y}
      isPreview={true}
      isHorizontal={wallPreview.wall.isHorizontal}
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
