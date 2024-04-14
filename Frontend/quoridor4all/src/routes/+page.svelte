<script lang="ts">
  let test: number = 5;

  let radius: number = 50;
  $: test2 = test;
  count.set(8);
  let countValue: number;
  count.subscribe((value) => {
    countValue = value;
  });
  import Test from "./Test.svelte";
  import CanvasComponentTest from "./canvasComponentTest.svelte";
  import Canvas from "./game/Canvas.svelte";
  import Wall from "./game/Wall.svelte";
  import { setConfigurations } from "./game/coordinateCalculation";
  import { count } from "./game/store";
  setConfigurations(5, 200, 1);
  let wallPreview: any = {
    wall: {
      isHorizontal: true,
      position: {
        x: 1,
        y: 1,
      },
    },
    isVisible: true,
  };

  function handleClick(event: any) {
    test++;
    test = 9;
    count.set(28);
    console.log($count, "count");
    radius = 40;

      wallPreview = {
      wall: {
        isHorizontal: false,
        position: {
          x: 1,
          y: 1,
        },
      },
      isVisible: true,
    };
  }

  let testFnc = (parameter: number) => {
    count.set(30);
    test = 80;
    radius = parameter;
      wallPreview = {
      wall: {
        isHorizontal: true,
        position: {
          x: 0,
          y: 0,
        },
      },
      isVisible: true,
    };
    console.log(radius);
  };
</script>

<p>test: {test}</p>
<p>test2: {test2}</p>
<p>count: {$count}</p>
<p>countValue: {countValue}</p>
<button on:click={handleClick}>Button 1</button>
<Test {testFnc}></Test>
<div>
  <Canvas onClick={() => {}} onResize={() => {}}>
    {#if radius > 10}
      <CanvasComponentTest {radius} />
    {/if}

    <Wall
      xBoard={wallPreview.wall.position.x}
      yBoard={wallPreview.wall.position.y}
      isPreview={true}
      isHorizontal={wallPreview.wall.isHorizontal}
    />
    <Wall
      xBoard={0}
      yBoard={0}
      isPreview={true}
      isHorizontal={false}
    />
  </Canvas>
</div>

<style>
  div {
    width: 100%;
    height: 100%;
  }
</style>
