<script lang="ts">
  import { onMount } from "svelte";
  import { players } from "../../store";
  import Board from "./Board.svelte";
  import { Button } from "flowbite-svelte";
  import PlayerViewHorizontal from "./PlayerViewHorizontal.svelte";
  import PlayerViewVertical from "./PlayerViewVertical.svelte";
  import revertPreview from './Board.svelte';

  function cancelMove(){
    // revertPreview();
  }
</script>

<div class="bg-gray-900">
  <div class="grid grid-cols-6 gap-4">
    <div class="col-start-2 col-span-4 text-center">
      <PlayerViewVertical
        playerName={$players[2].playerName}
        wallQuantity={$players[2].wallQuantity}
        color={$players[2].color}
      />
    </div>

    <div class="col-start-1 text-center">
      <PlayerViewHorizontal
        playerName={$players[1].playerName}
        wallQuantity={$players[1].wallQuantity}
        color={$players[1].color}
      />
    </div>

    <div class="col-start-2 col-span-4">
      <div class="aspect-w-1 aspect-h-1 bg-gray-200 border-8">
        <Board />
      </div>
    </div>

    <div class="col-start-6 text-center">
      <PlayerViewHorizontal
        playerName={$players[3].playerName}
        wallQuantity={$players[3].wallQuantity}
        color={$players[3].color}
      />
    </div>

    <div class="col-start-2 col-span-4 text-center">
      <PlayerViewVertical
        playerName={$players[0].playerName}
        wallQuantity={$players[0].wallQuantity}
        color={$players[0].color}
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
