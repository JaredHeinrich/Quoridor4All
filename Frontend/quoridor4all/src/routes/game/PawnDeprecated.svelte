<script lang="ts">
  export let color: any;
</script>

<div class="w-full aspect-[1/1] border-hidden border-4 absolute">
  <button
      class="pawn w-full border-gray-600 border-8"
			style="background: {color}"
		></button>
</div>

<style>
  	button {
		aspect-ratio: 1;
		border-radius: 50%;
		background: var(--color, #fff);
	}
</style>