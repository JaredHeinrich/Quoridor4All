<script lang="ts">
  import { Button, P } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import { goto } from "$app/navigation";
  import { players as playersStore, gameRunning } from "../store";

  const playerNames = ["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"];

  import { invoke } from "@tauri-apps/api/tauri";

  async function startGame() {
    playersStore.update((state) => {
      return state.map((player, index) => {
        return { ...player, playerName: playerNames[index] };
      });
    });
    gameRunning.set(true);
    let players: {
      player_name: string,
      pawn_color: string,
      pawn_side: string,
    }[] = $playersStore.map((player) => {
      return {
        player_name: player.playerName,
        pawn_color: "Red",
        pawn_side: "",
      };
    });
    players[0].pawn_side = "Bottom";
    players[1].pawn_side = "Left";
    players[2].pawn_side = "Top";
    players[3].pawn_side = "Right";
    
    let result = await invoke("start_game", { players });
    console.log(result);
    goto("/game");
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
      <Button
        color="dark"
        on:click={() => {
          goto("/rules");
        }}>Spielregeln</Button
      >
    </div>
  </div>
</form>
