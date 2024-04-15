<script lang=ts>
  import {Badge} from 'flowbite-svelte';
  export let wallQuantity: number = 6;
  export let playerName: string = "Player 1";
  export let color: any = "dark"
</script>

<div>
  <Badge color={color}>{playerName}</Badge>
  <Badge color={color}>Wände: {wallQuantity}</Badge>
</div>