<script lang="ts">
  import { onMount } from "svelte";
  import { players as playersStore } from "../../store";
  import Board from "./Board.svelte";
  import { Button } from "flowbite-svelte";
  import PlayerViewHorizontal from "./PlayerViewHorizontal.svelte";
  import PlayerViewVertical from "./PlayerViewVertical.svelte";
  import revertPreview from './Board.svelte';
  let currentPlayerIndex = 0;

  let players: any[] = [];
  const boardSize: number = 9;

  players = [
    {
      position: {
        x: 4,
        y: 8,
      },
      number_of_available_walls: 6,
      goal: {
        y: 0,
      },
      player_name: "Player 1",
      color: "red",
    },
    {
      position: {
        x: 0,
        y: 4,
      },
      number_of_available_walls: 6,
      goal: {
        x: 8,
      },
      player_name: "Player 2",
      color: "yellow",
    },
    {
      position: {
        x: 4,
        y: 0,
      },
      number_of_available_walls: 6,
      goal: {
        y: 8,
      },
      player_name: "Player 3",
      color: "blue",
    },
    {
      position: {
        x: 8,
        y: 4,
      },
      number_of_available_walls: 6,
      goal: {
        x: 0,
      },
      player_name: "Player 4",
      color: "green",
    },
  ];
  let walls: any[] = [
    {
      position: {
        x: 5,
        y: 5,
      },
      isHorizontal: true,
    },
    {
      position: {
        x: 0,
        y: 0,
      },
      isHorizontal: false,
    },
    {
      position: {
        x: 7,
        y: 4,
      },
      isHorizontal: true,
    },
    {
      position: {
        x: 1,
        y: 7,
      },
      isHorizontal: false,
    },
    {
      position: {
        x: 4,
        y: 2,
      },
      isHorizontal: true,
    },
  ];

  function cancelMove(){
    revertPreview();
  }

  onMount(() => {
    //fetch data from backend, players should be created in backend

    players = $playersStore.map((player) => {
      return {
        position: {
          x: 0,
          y: 0
        },
        number_of_available_walls: 6,
        goal: null,
        player_name: player,
      };
    });

    console.log(players);
  });
</script>

<div class="bg-gray-900">
  <div class="grid grid-cols-6 gap-4">
    <div class="col-start-2 col-span-4 text-center">
      <PlayerViewVertical
        player_name={players[2].player_name}
        number_of_available_walls={players[2].number_of_available_walls}
        color={players[2].color}
      />
    </div>

    <div class="col-start-1 text-center">
      <PlayerViewHorizontal
        player_name={players[1].player_name}
        number_of_available_walls={players[1].number_of_available_walls}
        color={players[1].color}
      />
    </div>

    <div class="col-start-2 col-span-4">
      <div class="aspect-w-1 aspect-h-1 bg-gray-200 border-8">
        <Board size={boardSize} players={players} walls={walls} bind:currentPlayerIndex />
      </div>
    </div>

    <div class="col-start-6 text-center">
      <PlayerViewHorizontal
        player_name={players[3].player_name}
        number_of_available_walls={players[3].number_of_available_walls}
        color={players[3].color}
      />
    </div>

    <div class="col-start-2 col-span-4 text-center">
      <PlayerViewVertical
        player_name={players[0].player_name}
        number_of_available_walls={players[0].number_of_available_walls}
        color={players[0].color}
      />
    </div>
  </div>
  <div class="border-8 text-center border-gray-900">
    <Button color="red" on:click={cancelMove}>Abbrechen</Button>
    <Button color="blue" on:click={()=>{}}>Bestätigen</Button>
  </div>
  <div class="border-8 text-center border-gray-900">
    <Button color="dark">Zurück</Button>
    <Button color="dark">Spiel abbrechen</Button>
    <Button color="dark">Spielregeln</Button>
  </div>
</div>
