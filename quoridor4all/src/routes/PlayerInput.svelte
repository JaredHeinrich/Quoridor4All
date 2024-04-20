<script lang="ts">
  import { Input, Label} from 'flowbite-svelte';
  import { playerNames } from "../store"

  export let playerNumber: number;
</script>
    
<div>
  <Label for="player_name" class="mb-2 text-gray-200">Spieler {playerNumber}</Label>
  <Input bind:value={$playerNames[playerNumber - 1 ]} type="text" id="player_name" placeholder="Spieler {playerNumber}" required />
</div>
