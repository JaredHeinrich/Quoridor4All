<script lang="ts">
  import { Button } from "flowbite-svelte";
  import PlayerInput from "./PlayerInput.svelte";
  import {goto} from '$app/navigation';
  import { players as playersStore } from '../../store';

  const players = ["Spieler 1", "Spieler 2", "Spieler 3", "Spieler 4"];

  async function startGame(){
    playersStore.set(players);
    try{
    const response = await fetch("http://localhost:8080/api/auth/me", {
      method: "POST",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      //body?
    });
    if (response.ok) {
      goto('/game');
    } else {
      console.error("connection failed:", response);
    }
  } catch (error){
      console.log("no backend connected");
      console.error(error);
     
  }
    //till no backend connected
    goto('/game');
  }

</script>

<form class="p-10">
  <div class="grid gap-6 mb-6 md:grid-cols-1">
    {#each players as player, i}
      <div>
        <PlayerInput bind:playerName={players[i]} playerNumber={i + 1} />
      </div>
    {/each}

    <div>
      <Button type="button" on:click={startGame}>Start</Button>
    </div>
  </div>
</form>
