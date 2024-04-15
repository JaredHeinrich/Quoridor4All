<script lang="ts">
  import { getContext, onDestroy, onMount } from "svelte";

  export let radius: number;
  const { register, unregister} = getContext<{ register: (fn: any) => void, unregister: (fn: any) => void }>('Canvas');

onMount(() => {
  register(draw);

  return () => {
  }
});

onDestroy(() => {
    unregister(draw);
  });
function draw(ctx : CanvasRenderingContext2D) {
    console.log("draw function test component")
    ctx.beginPath();
    ctx.fillStyle = "red";
    ctx.arc(
      50, 
      50,
      radius, 
      0, 2*Math.PI, true );
    ctx.fill();
  }
</script>