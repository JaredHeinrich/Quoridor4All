<script lang="ts">
  import { updated } from "$app/stores";
  import { onMount, setContext } from "svelte";

  export let width: number;

  let canvas;
  const drawFunctions = [];


  setContext('Canvas', {
    register(drawFn) {
      drawFunctions.push(drawFn);
    },
    unregister(drawFn){
      drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
    }
  });

  onMount(() => {
    const ctx = canvas.getContext('2d');
    canvas.width = width;
    canvas.height = width;

    function update() {
      //draw something
      ctx.clearRect(0,0, canvas.width, canvas.height)

      drawFunctions.forEach(drawFn => {
        drawFn(ctx);
      })

      frameId = requestAnimationFrame(update);
    }

    let frameId = requestAnimationFrame(update);

    return () => {
      cancelAnimationFrame(update);
    }
  })
</script>

<canvas bind:this={canvas} />
<slot />