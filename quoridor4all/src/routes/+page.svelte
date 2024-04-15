<script lang="ts">
  import { Button } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import {goto} from '$app/navigation';
  import { players as playersStore } from '../store';

  const playerNames = ["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"];

  async function startGame(){
    //update playerNames
    // playersStore.update();

    //till no backend connected
    goto('/game');
  }

</script>

<form class="p-10">
  <div class="grid gap-6 mb-6 md:grid-cols-1">
    {#each playerNames as player, i}
      <div>
        <PlayerInput bind:playerName={playerNames[i]} playerNumber={i + 1} />
      </div>
    {/each}

    <div>
      <Button type="button" on:click={startGame}>Start</Button>
    </div>
  </div>
</form>
