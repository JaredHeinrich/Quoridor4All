<script lang="ts">
  import { Button, P } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import {startGame, toRules} from "./navigation";
  import { players } from "../store";
</script>

<form class="p-10 bg-gray-900 text-gray-200">
  <div class="grid gap-6 mb-6 md:grid-cols-1">
    {#each $players as player, i}
      <div>
        <PlayerInput playerNumber={i + 1} />
      </div>
    {/each}

    <div>
      <Button type="button" on:click={startGame}>Start</Button>
      <Button
        color="dark"
        on:click={toRules}>Spielregeln</Button
      >
    </div>
  </div>
</form>
