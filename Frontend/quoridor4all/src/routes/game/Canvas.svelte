<script lang="ts">
  import { updated } from "$app/stores";
  import { onMount, setContext } from "svelte";

  export let width: number;

  let canvas;
  const drawFunctions = [];

  $: {
    console.log(canvas);
    console.log(width);
  }

  setContext("Canvas", {
    register(drawFn) {
      drawFunctions.push(drawFn);
    },
    unregister(drawFn) {
      drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
    },
  });

  export let onClick;
  function handleClick(event) {
    let boundingRect = canvas.getBoundingClientRect(); 
    let clickPosition = {
      x: event.clientX - boundingRect.left,
      y: event.clientY - boundingRect.top
    };
    console.log("clicked", clickPosition)
    onClick(clickPosition);
  }

  onMount(() => {
    const ctx = canvas.getContext("2d");
    // canvas.width = canvas.parentElement.offsetWidth;
    // canvas.height = canvas.parentElement.offsetWidth;
    canvas.width = width;
    canvas.height = width;

    canvas.addEventListener("click", handleClick);
    

    function update() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      //draw something
      drawFunctions.forEach((drawFn) => {
        drawFn(ctx);
      });

      frameId = requestAnimationFrame(update);
    }

    let frameId = requestAnimationFrame(update);

    return () => {
      cancelAnimationFrame(update);
      canvas.removeEventListener('click', handleClick);
    };
  });
</script>

<canvas bind:this={canvas} on:click={handleClick} />
<slot />
