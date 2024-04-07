<script lang="ts">
  import { getContext, onMount } from "svelte";

  export let centerX: number;
  export let centerY: number;
  export let radius: number; 
  export let color: string;

  const { register, unregister} = getContext<{ register: (fn: any) => void, unregister: () => void }>('Canvas');

  onMount(() => {
    register(draw);

    return () => {
      unregister();
    }
  });

  function draw(ctx : CanvasRenderingContext2D) {
    ctx.beginPath();
    ctx.fillStyle = color;
    ctx.arc(centerX, centerY, radius, 0, 2*Math.PI, true );
    ctx.fill();
  }
</script>