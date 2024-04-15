<script lang="ts">
  import Canvas from "./game/Canvas.svelte";
  import Pawn from "./game/Pawn.svelte";
  import Square from "./game/Square.svelte";
  import Wall from "./game/Wall.svelte";
  import { setConfigurations } from "./game/coordinateCalculation";
  import { getPossiblePlayerMoves, canvasClick } from "./game/gameLogic";

</script>