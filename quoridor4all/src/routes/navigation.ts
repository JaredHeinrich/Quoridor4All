
import { invoke } from "@tauri-apps/api/tauri";
import { players, gameRunning, playerNames } from "../store";
import { get } from "svelte/store";
import { goto } from "$app/navigation";
import { initializeGame } from "./game/gameLogic";

export async function startGame() {
  initializeGame();
  players.update((state) => {
    return state.map((player, index) => {
      return { ...player, playerName: get(playerNames)[index] };
    });
  });
  gameRunning.set(true);

  const playersBackend: {
    player_name: string,
    pawn_color: string,
    pawn_side: string,
  }[] = get(players).map((player) => {
    return {
      player_name: player.playerName,
      pawn_color: "Red",
      pawn_side: "",
    };
  });
  playersBackend[0].pawn_side = "Bottom";
  playersBackend[1].pawn_side = "Left";
  playersBackend[2].pawn_side = "Top";
  playersBackend[3].pawn_side = "Right";
  
  let result = await invoke("start_game", { players: playersBackend });
  console.log(result);
  goto("/game");
}

export function toRules(){
  goto("/rules");
}

export function continueGame(): void{
  if(get(gameRunning)){
    goto("/game");
  }else{
    goto("/");
  }
}

export function cancelGame(): void{
  gameRunning.set(false);
  goto("/");
}