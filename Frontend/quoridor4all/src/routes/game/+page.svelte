<script lang="ts">
  import { onMount } from "svelte";
  import { players as playersStore } from "../../store";
  import Board from "./Board.svelte";

  let players: any = null;
  const boardSize: number = 7;


  onMount(() => {
    //fetch data from backend, players should be created in backend

    players = $playersStore.map((player) => {
      return {
        xPosition: null, 
        yPosition: null,
        number_of_available_walls: 6, 
        goal: null, 
        player_name: player,
      };
    });

    console.log(players);
  });
</script>

<div>
  <div class="grid grid-cols-4 grid-rows-6 gap-4">
    <div class="col-start-2 row-start-1 col-span-2 text-center">
      <p>Spieler 3</p>
    </div>
  
    <div class="col-start-1 row-start-2 text-center">
      <p>Spieler 2</p>
    </div>
  
    <div class="col-start-2 row-start-2 col-span-2 row-span-4">
      <div class="aspect-w-1 aspect-h-1 bg-gray-200">
        <!-- <p class="m-4 text-center">Brett</p> -->
        <Board />
      </div>
    </div>
  
    <div class="col-start-4 row-start-2 text-center">
      <p>Spieler 4</p>
    </div>
  
    <div class="col-start-2 row-start-6 col-span-2 text-center">
      <p>Spieler 1</p>
    </div>
  </div>
</div>
