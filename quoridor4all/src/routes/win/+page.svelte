<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { cancelGame } from "../navigation";
  import { winnerName } from "../../store";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let leaderBordNames: string[] = [];

  onMount(async ()=>{
    leaderBordNames = await getTopPlayers();
    console.log(await getTopPlayers());
  })

  async function getTopPlayers(): Promise<string[]> {
    console.log("Backend: ", await invoke("get_top_players"))
    return await invoke("get_top_players");
  }
</script>

<div class="text-white p-8 rounded-lg text-center">
  <p class="text-lg">Der Sieger ist:</p>
  <p class="text-4xl font-bold text-gradient"> {$winnerName} 🎉</p>
  <img src='/winning.gif' alt='Congratulations image' class="mx-auto mt-4" aria-hidden="true" />
</div>

<ul class="bg-gray-900 text-white p-8 rounded-lg mt-8">
  <p class="text-lg">All Time Best:</p>
  {#each leaderBordNames as player, i }
    <li class="{i === 0 ? 'text-lg text-red-500' : i === 1 ? 'text-base' : 'text-sm'}">{player} Siege: {player}</li>
  {/each}
</ul>
<div class="border-8 text-center border-gray-900">
  <Button color="dark" on:click={cancelGame}>Startseite</Button>
</div>
