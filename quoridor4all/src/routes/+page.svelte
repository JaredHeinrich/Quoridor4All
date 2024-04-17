<script lang="ts">
  import { Button } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import {goto} from '$app/navigation';
  import { players, gameRunning } from '../store';

  const playerNames = ["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"];

  function startGame(){
    players.update(state => {
      return state.map((player, index) => {
        return {...player, playerName: playerNames[index]}
      });
    });
    gameRunning.set(true);
    goto('/game');
  }

</script>

<form class="p-10 bg-gray-900 text-gray-200">
  <div class="grid gap-6 mb-6 md:grid-cols-1">
    {#each playerNames as player, i}
      <div>
        <PlayerInput bind:playerName={playerNames[i]} playerNumber={i + 1} />
      </div>
    {/each}

    <div>
      <Button type="button" on:click={startGame}>Start</Button>
      <Button color="dark" on:click={()=>{goto("/rules")}} >Spielregeln</Button>
    </div>
  </div>
</form>
