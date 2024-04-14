<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import Pawn from "./Pawn.svelte";
  import Square from "./Square.svelte";
  import Wall from "./Wall.svelte";
  import {setConfigurations,
  } from "./coordinateCalculation";
  import {
    getPossiblePlayerMoves,
    checkWallObstacle,
    isWallPositionValid,
    canvasClick,
  } from "./gameLogic";

  export let size: number = 9;
  export let players: any;
  export let walls: any;
  export let currentPlayerIndex: number;

  let squareWidthComparedToWallWidth = 4; // 4 times bigger squares than walls

  function onResize(width: number){
    console.log("handle resize", width)
    setConfigurations(size, width, squareWidthComparedToWallWidth);
  }

  let grid = new Array(size).fill(0).map(() => new Array(size).fill(0));

  let previewPlayers: any = [];
  let wallPreview: any = {
    wall: {
      isHorizontal: true,
      position: {
        x: 0,
        y: 0,
      },
    },
    isVisible: false,
  };

  console.log("Wall test", isWallPositionValid(wallPreview.wall, size, walls));

  function handleClick(clickPosition: any) {
    let canvasWidth = document.getElementById("outerDiv")?.offsetWidth  ?? 500;//div width or width inside of the canvas/inside configuration or last call onResize;
    let clickObject = canvasClick(
      clickPosition,
      canvasWidth,
      size,
      walls,
      players
    ) ?? { isValidClick: false };

    console.log("clickObject", clickObject);
    if (!clickObject.isValidClick) {
      return;
    }

    if (clickObject.clickedWall) {
      wallPreview = {
        wall: clickObject.clickedWall,
        isVisible: true,
      };
      console.log("wallPreview", wallPreview);

      previewPlayers.forEach((previewPlayer: any) =>{
        previewPlayer.isVisible = false;
      });
      return;
    }

    if (clickObject.clickedPawn) {
    }
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

<div id ="outerDiv">
  <Canvas onClick={handleClick} onResize={onResize}>
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
      {#if previewPlayer.isVisible}
        <Pawn
          xBoard={previewPlayer.position.x}
          yBoard={previewPlayer.position.y}
          color={players[previewPlayer.playerIndex].color}
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
