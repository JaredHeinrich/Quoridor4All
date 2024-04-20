<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { continueGame, cancelGame} from "../navigation";
  
</script>
<div class="bg-gray-900 text-gray-200 p-4">
  <h2 class="text-2xl font-bold mb-4">Spielregeln</h2>
  <p class="mb-10 text-justify" >
    Quoridor ist ein Brettspiel, dass mit zwei oder vier Personen gespielt werden kann. Klassischerweise ist das Spielbrett ein 9x9-Feld. Jeder Spieler startet mit seiner Spielfigur auf dem Mittelfeld einer Randspalte. Zusätzlich erhält jeder Spieler 10 Wände der länge 2 Spielfelder (bei 4 Spielern 6 Wände). Das Ziel ist es, die gegenüberliegende Randseite zu erreichen. Dafür ziehen die Spieler abwechselnd, beziehungsweise im Uhzeigersinn. Ein Zug beinhält entweder eine Bewegung der eigenen Spielfigur um eins in einer geraden Spielfeldrichtung (nicht schräg) oder das Setzen einer Wand zwischen 4 Felder. Eine Wand kann nicht durch eine Spielfigur passiert werden. Eine Wand kann nicht eine andere Wand schneiden. Auch dürfen Wände nicht so gesetzt werden, dass einer Spielfigur es nicht mehr möglich ist, die andere Seite zu erreichen. Es gewinnt, wessen Spielfigur als erstes die andere Seite erreicht.
  </p>
  <!-- <a href="https://de.wikipedia.org/wiki/Quoridor#:~:text=Quoridor%20ist%20ein%20Strategiespiel%20f%C3%BCr,der%20Great%20American%20Trading%20Company.">Mehr Infos</a> -->
  <Button color="purple" on:click={()=>open('https://de.wikipedia.org/wiki/Quoridor#:~:text=Quoridor%20ist%20ein%20Strategiespiel%20f%C3%BCr,der%20Great%20American%20Trading%20Company.')}>Mehr Infos</Button>
  <div class="border-8 text-center border-gray-900">
    <Button color="dark" on:click={continueGame}>Weiter</Button>
    <Button color="dark" on:click={cancelGame}>Spiel abbrechen</Button>
  </div>
</div>