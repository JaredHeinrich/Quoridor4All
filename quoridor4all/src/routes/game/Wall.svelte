<script lang="ts">
  import { getContext, onDestroy, onMount } from "svelte";
  import {
    startOfSquare,
    endOfSquare,
    squareWidthCanvas,
    wallWidthCanvas,
  } from "./coordinateCalculation";

  export let xBoard: number;
  export let yBoard: number;
  export let isHorizontal: boolean;
  export let isPreview: boolean;

  let color: string;

  if (isPreview) {
    color = "rgba(84, 47, 2, 0.6)";
  } else {
    color = "rgb(84, 47, 2)";
  }

  const { register, unregister } = getContext<{
    register: (fn: any) => void;
    unregister: (fn: any) => void;
  }>("Canvas");

  onMount(() => {
    register(draw);

    return () => {
    };
  });

  onDestroy(() => {
    unregister(draw);
  });

  function draw(ctx: CanvasRenderingContext2D) {
    let topLeftCornerX: number;
    let topLeftCornerY: number;
    let width: number;
    let height: number;

    if (isHorizontal) {
      topLeftCornerX = startOfSquare(xBoard);
      topLeftCornerY = endOfSquare(yBoard);
      height = wallWidthCanvas;
      width = 2 * squareWidthCanvas + wallWidthCanvas;
    } else {
      topLeftCornerX = endOfSquare(xBoard);
      topLeftCornerY = startOfSquare(yBoard);
      height = 2 * squareWidthCanvas + wallWidthCanvas;
      width = wallWidthCanvas;
    }

    ctx.beginPath();
    ctx.fillStyle = color;
    ctx.rect(topLeftCornerX, topLeftCornerY, width, height);
    ctx.fill();

    // console.log("end draw wall")
  }
</script>
