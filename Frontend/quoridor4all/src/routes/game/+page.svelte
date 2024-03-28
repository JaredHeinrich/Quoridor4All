<script lang="ts">
  import { onMount } from "svelte";
  import { players as playersStore } from "../../store";
  
  let players: any = null;



  onMount(() => {
    //fetch data from backend, players should be created in backend

    players = $playersStore.map((player) => {
      return {
        xPosition: null, 
        yPosition: null,
        number_of_available_walls: 6, 
        goal: null, 
        player_name: player,
      };
    });

    console.log(players);
  });
</script>

<div>
  <h1>Heading</h1>
</div>
