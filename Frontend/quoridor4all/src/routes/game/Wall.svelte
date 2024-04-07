<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { startOfSquare, endOfSquare, squareWidthCanvas, wallWidthCanvas} from './coordinateCalculation';

  export let xBoard: number;
  export let yBoard: number;
  export let isVertical: boolean;
  export let isPreview: boolean;

  let topLeftCornerX: number;
  let topLeftCornerY: number;
  let width: number; 
  let height: number;
  
  if(isVertical){
    topLeftCornerX=startOfSquare(xBoard);
    topLeftCornerY=endOfSquare(yBoard);
    height=wallWidthCanvas;
    width= 2*squareWidthCanvas + wallWidthCanvas;
  }else{
    topLeftCornerX=endOfSquare(xBoard);
    topLeftCornerY=startOfSquare(yBoard);
    height =  2*squareWidthCanvas + wallWidthCanvas;
    width =   wallWidthCanvas;
  }

  let color: string;

  if(isPreview){
    color = 'rgba(84, 47, 2, 0.6)'
  } else{
    color = 'rgb(84, 47, 2)'
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