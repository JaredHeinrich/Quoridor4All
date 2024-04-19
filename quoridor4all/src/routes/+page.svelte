<script lang="ts">
  import { Button, P } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import {startGame, toRules} from "./navigation";
  
  function start(){
    startGame(playerNames);
  }

  const playerNames = ["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"];
</script>

<form class="p-10 bg-gray-900 text-gray-200">
  <div class="grid gap-6 mb-6 md:grid-cols-1">
    {#each playerNames as player, i}
      <div>
        <PlayerInput bind:playerName={playerNames[i]} playerNumber={i + 1} />
      </div>
    {/each}

    <div>
      <Button type="button" on:click={start}>Start</Button>
      <Button
        color="dark"
        on:click={toRules}>Spielregeln</Button
      >
    </div>
  </div>
</form>
