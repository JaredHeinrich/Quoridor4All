<script lang="ts">
  import { getContext, onMount } from "svelte";

  export let topLeftCornerX: number;
  export let topLeftCornerY: number;
  export let width: number; //square width is equal to height

  const { register, unregister} = getContext<{ register: (fn: any) => void, unregister: () => void }>('Canvas');

  onMount(() => {
    register(draw);

    return () => {
      unregister();
    }
  });

  function draw(ctx : CanvasRenderingContext2D) {
    ctx.beginPath();
    ctx.fillStyle = 'gray';
    ctx.rect(topLeftCornerX, topLeftCornerY, width, width);
    ctx.fill();
  }
</script>