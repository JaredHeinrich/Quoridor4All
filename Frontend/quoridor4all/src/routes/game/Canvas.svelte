<script lang="ts">
  import { onMount, setContext } from "svelte";

  let width: number;

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
    console.log("handleClick in Canvas", canvas);
    onClick(clickPosition);
    console.log("handleClick in Canvas after onClick", canvas);
  }

  export let onResize: Function;

  function handleResize() {
    if(canvas && canvas.parentElement){
      width = canvas.parentElement.offsetWidth;
      canvas.width = width;
      canvas.height = width;
      onResize(width);
      
    }
  }

  onMount(() => {
    console.log("canvas on mount begin")
    const ctx = canvas.getContext("2d");

    canvas.addEventListener("click", handleClick);
    window.addEventListener("resize", handleResize)
    
    handleResize();

    let update: Function = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      //draw something
      drawFunctions.forEach((drawFn) => {
        drawFn(ctx);
      });

      frameId = requestAnimationFrame(update as FrameRequestCallback );
    }

    let frameId = requestAnimationFrame(update as FrameRequestCallback);
    return () => {
      cancelAnimationFrame(frameId);
      window.removeEventListener('resize', handleResize);
      canvas.removeEventListener('click', handleClick);
    };
  });
</script>

<canvas bind:this={canvas} on:click={handleClick} />
<slot />
