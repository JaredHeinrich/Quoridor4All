<script lang="ts">
  import { onMount } from "svelte";
  import { players as playersStore } from "../../store";
  import Board from "./Board.svelte";
  import { Button } from "flowbite-svelte";
  import { text } from "@sveltejs/kit";
  import PlayerViewHorizontal from "./PlayerViewHorizontal.svelte";

  let players: any = null;
  const boardSize: number = 9;

  players = [{
        position: {
          x: 8, 
          y: 4,
        },
        number_of_available_walls: 6, 
        goal: {
          x: 0,
        }, 
        player_name: "Player 1",
      }, {
        position: {
          x: 4, 
          y: 0,
        },
        number_of_available_walls: 6, 
        goal: {
          y: 8,
        }, 
        player_name: "Player 2",
      }, {
        position: {
          x: 0, 
          y: 4,
        },
        number_of_available_walls: 6, 
        goal: {
          x: 8,
        }, 
        player_name: "Player 3",
      }, {
        position: {
          x: 4, 
          y: 8,
        },
        number_of_available_walls: 6, 
        goal: {
          y: 0,
        },  
        player_name: "Player 4",
      }
    ]

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
      <PlayerViewHorizontal player_name={players[1].player_name} number_of_available_walls={players[1].number_of_available_walls} color="dark"/>
    </div>
  
    <div class="col-start-2 row-start-2 col-span-2 row-span-4">
      <div class="aspect-w-1 aspect-h-1 bg-gray-200">
        <!-- <p class="m-4 text-center">Brett</p> -->
        <Board />
      </div>
    </div>
  
    <div class="col-start-4 row-start-2 text-center">
      <PlayerViewHorizontal player_name={players[3].player_name} number_of_available_walls={players[3].number_of_available_walls} color="dark"/>
    </div>
  
    <div class="col-start-2 row-start-6 col-span-2 text-center">
      <p>Spieler 1</p>
    </div>
  </div>
  <div>
    <Button color="dark">Zurück</Button>
    <Button color="dark">Spiel abbrechen</Button>
    <Button color="dark">Spielregeln</Button>
  </div>
</div>
