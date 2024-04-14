<script lang=ts>
  import {Badge} from 'flowbite-svelte';
  export let number_of_available_walls: number = 6;
  export let player_name: string = "Player 1";
  export let color: any = "dark"
</script>

<div>
  <Badge color={color}>{player_name}</Badge>
  <Badge color={color}>Wände: {number_of_available_walls}</Badge>
</div>