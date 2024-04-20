<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { cancelGame } from "../navigation";
  import { winnerName } from "../../store";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let leaderBordNames: {name: string, wins: number}[] = [];

  onMount(async ()=>{
    leaderBordNames = await getTopPlayers();
    console.log(await getTopPlayers());
  })

  async function getTopPlayers(): Promise<{name: string, wins: number}[]> {
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
  <p class="text-lg w-screen flex items-center justify-center">All Time Best:</p>
  {#each leaderBordNames as player, i }
  <div class="w-screen flex justify-center items-center ">
    <div class="flex justify-center items-center text-center bg-gray-700 rounded shadow m-4 {i === 0 ? 'w-1/5 bg-orange-800' : i === 1 ? 'w-1/3' : 'w-1/2'}">
      <li class="">{player.name} Siege: {player.wins}</li>
  </div>
</div>
  {/each}
</ul>
<div class="border-8 text-center border-gray-900">
  <Button color="dark" on:click={cancelGame}>Startseite</Button>
</div>
