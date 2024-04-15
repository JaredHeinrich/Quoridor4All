<script lang="ts">
  import { getContext, onDestroy, onMount } from "svelte";
  import { startOfSquare, squareWidthCanvas} from './coordinateCalculation';

  export let xBoard: number;
  export let yBoard: number;

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
    ctx.beginPath();
    ctx.fillStyle = 'gray';
    ctx.rect(
      startOfSquare(xBoard), 
      startOfSquare(yBoard),
      squareWidthCanvas,
      squareWidthCanvas);
    ctx.fill();
  }
</script>