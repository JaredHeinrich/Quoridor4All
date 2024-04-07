<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { startOfSquare, squareWidthCanvas} from './coordinateCalculation';

  export let xBoard: number;
  export let yBoard: number;
  let width: number = squareWidthCanvas; //square width is equal to height

  const { register, unregister} = getContext<{ register: (fn: any) => void, unregister: () => void }>('Canvas');

  onMount(() => {
    register(draw);

    return () => {
      unregister();
    }
  });

  function draw(ctx : CanvasRenderingContext2D) {
    ctx.beginPath();
    ctx.fillStyle = 'gray';
    ctx.rect(
      startOfSquare(xBoard), 
      startOfSquare(yBoard),
      width,
      width);
    ctx.fill();
  }
</script>