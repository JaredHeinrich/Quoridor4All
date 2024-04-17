<script lang="ts">
  import { onMount, setContext } from "svelte";
  import { showClickedPreview } from "./gameLogic";

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

  function handleClick(event: any) {
    let boundingRect = canvas.getBoundingClientRect(); 
    let clickPosition = {
      x: event.clientX - boundingRect.left,
      y: event.clientY - boundingRect.top
    };    
    let canvasWidth: number = canvas.parentElement.offsetWidth; //div width or width inside of the canvas/inside configuration or last call onResize;
    showClickedPreview(clickPosition, canvasWidth);
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
