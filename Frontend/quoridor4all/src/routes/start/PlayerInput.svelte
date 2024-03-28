<script lang="ts">
  import { Input, Label, Button } from 'flowbite-svelte';

  export let playerNumber: number;
  export let playerName: string;
</script>
    
<div>
  <Label for="player_name" class="mb-2">Spieler {playerNumber}</Label>
  <Input bind:value={playerName} type="text" id="player_name" placeholder="Spieler {playerNumber}" required />
</div>
