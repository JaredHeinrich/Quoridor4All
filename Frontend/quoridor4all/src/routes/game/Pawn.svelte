<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { centerOfSquare, squareWidthCanvas} from './coordinateCalculation';

  export let xBoard: number;
  export let yBoard: number;
  export let color: string;
  export let isPreview: boolean;

  if(isPreview){
    color = 'rgba(84, 47, 2, 0.6)'
  } else{
    color = color
  }

  let radius: number = (squareWidthCanvas / 2) * 0.8; 

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
    ctx.arc(
      centerOfSquare(xBoard), 
      centerOfSquare(yBoard),
      radius, 
      0, 2*Math.PI, true );
    ctx.fill();
  }
</script>