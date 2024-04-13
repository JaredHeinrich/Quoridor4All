<script lang="ts">
  import { updated } from "$app/stores";
  import { onMount, setContext } from "svelte";

  export let width: number;

  let canvas: any;
  const drawFunctions: Function[] = [];

  setContext("Canvas", {
    register(drawFn: Function) {
      drawFunctions.push(drawFn);
    },
    unregister(drawFn: Function) {
      drawFunctions.splice(drawFunctions.indexOf(drawFn), 1);
    },
  });

  export let onClick: Function;

  function handleClick(event: any) {
    let boundingRect = canvas.getBoundingClientRect(); 
    let clickPosition = {
      x: event.clientX - boundingRect.left,
      y: event.clientY - boundingRect.top
    };
    onClick(clickPosition);
  }

  function handleResize(event: any) {
    if(canvas && canvas.parentElement){
      width = canvas.parentElement.offsetWidth;
      canvas.width = width;
      canvas.height = width;
      console.log("handle resize", width)

    }
  }

  onMount(() => {
    const ctx = canvas.getContext("2d");
    // canvas.width = canvas.parentElement.offsetWidth;
    // canvas.height = canvas.parentElement.offsetWidth;
    canvas.width = width;
    canvas.height = width;

    canvas.addEventListener("click", handleClick);
    window.addEventListener("resize", handleResize)
    

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
      window.removeEventListener('resize', handleResize);
      canvas.removeEventListener('click', handleClick);
    };
  });
</script>

<canvas bind:this={canvas} on:click={handleClick} />
<slot />
