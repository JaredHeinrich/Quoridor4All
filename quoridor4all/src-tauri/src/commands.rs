use tauri::State;

use crate::{structs::game::{Game, Player}, GameState, BOARD_SIZE, NUMBER_OF_PLAYERS, NUMBER_OF_WALLS_PER_PLAYER};

#[tauri::command]
pub async fn start_game<'a>(players: [Player; NUMBER_OF_PLAYERS], state: State<'a, GameState>) -> Result<(), String> {
    let mut game = state.game.lock().await;
    *game = Some(Game::new(BOARD_SIZE, NUMBER_OF_WALLS_PER_PLAYER, players));
    Ok(())
}

#[tauri::command]
pub async fn get_player_names<'a>(state: State<'a, GameState>) -> Result<Vec<(String,)>, sqlx::Error> {
    let pool = state.db_pool.lock().await;
    let result: Vec<(String,)> = sqlx::query_as("SELECT name FROM players").fetch_all(&*pool).await?;
    Ok(result)
}
