<script lang="ts">
  import { getContext, onMount } from "svelte";

  export let topLeftCornerX: number;
  export let topLeftCornerY: number;
  export let width: number; //square width is equal to height
  export let height: number;
  export let isPreview: boolean;

  let color: string;

  if(isPreview){
    color = 'rgb(84 47 2 / 0,6)'
  } else{
    color = 'rgb(84 47 2 / 1)'
  }

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
    ctx.rect(topLeftCornerX, topLeftCornerY, width, height);
    ctx.fill();
  }
</script>