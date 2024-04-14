<script lang="ts">
  export let testFnc: Function;
  function handleClick(){
    testFnc(10);
  }
</script>

<button on:click={handleClick}>Button 2</button>
