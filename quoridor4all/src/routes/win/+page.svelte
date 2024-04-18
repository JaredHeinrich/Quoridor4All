<script lang="ts">
  let currentWinnerName: string = "Doris86";
  let leaderBordNames = ["Player 1", "Player 2", "Player 3"];
</script>

<style>
  .winner {
    font-size: 3em;
    color: red;
    text-align: center;
  }
  
  .mostWins {
    font-size: 2em;
    list-style-type: none;
    text-align: center;
  }
  
  .mostWins li {
    margin: 10px auto;
  }
</style>

<div class="winner">
  Der Sieger ist: {currentWinnerName} 🎉
  <img src='/img/winning.gif' alt='Congratulations image' />
</div>

<ul class="mostWins">
  Die 3 häufigsten Gewinner sind:
  {#each leaderBordNames as player }
    <li>{player}</li>
  {/each}
</ul>
