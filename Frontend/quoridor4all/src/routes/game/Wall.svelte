<script lang="ts">
  export let colStart: number;
  export let rowStart: number;
  export let height: number;
  export let width: number;
</script>

<div
  class="col-start-{colStart} row-start-{rowStart} col-span-{height} row-span-{width} bg-brown-800"
>
  <div class="w-full h-full bg-brown-800"></div>
</div>
