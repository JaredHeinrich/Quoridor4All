<script lang="ts">
  import { getContext, onDestroy, onMount } from "svelte";
  import { centerOfSquare, squareWidthCanvas} from './coordinateCalculation';

  export let xBoard: number;
  export let yBoard: number;
  export let color: string;
  export let isPreview: boolean;

  let colorMap: { [key: string]: string } = {
  "blue": "0, 0, 200",
  "red": "200, 0, 0",
  "green": "0, 200, 0",
  "yellow": "200, 200, 0"
  };

  if(isPreview){
   color = 'rgba(' + colorMap[color] + ', 0.3)';
} else {
   color = 'rgb(' + colorMap[color] + ')';
}

  

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
    let radius: number = (squareWidthCanvas / 2) * 0.8; 
    
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