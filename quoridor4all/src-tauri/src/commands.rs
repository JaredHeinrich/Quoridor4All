use tauri::State;

use crate::{structs::game::{Game, Player}, GameState, BOARD_SIZE, NUMBER_OF_PLAYERS, NUMBER_OF_WALLS_PER_PLAYER};

#[tauri::command]
pub async fn start_game<'a>(players: [Player; NUMBER_OF_PLAYERS], state: State<'a, GameState>) -> Result<(), String> {
    let pool = state.db_pool.lock().await;
    let _result = sqlx::query("
                INSERT OR IGNORE INTO players (names, wins)
                VALUES ($1, 0), ($2, 0), ($3, 0), ($4, 0);
                ")
        .bind(&players[0].player_name)
        .bind(&players[1].player_name)
        .bind(&players[2].player_name)
        .bind(&players[3].player_name)
        .execute(&*pool)
        .await;
    let mut game = state.game.lock().await;
    *game = Some(Game::new(BOARD_SIZE, NUMBER_OF_WALLS_PER_PLAYER, players));
    Ok(())
}

#[tauri::command]
pub async fn get_player_names<'a>(state: State<'a, GameState>) -> Result<Vec<String>, String> {
    let pool = state.db_pool.lock().await;
    let result: Vec<String> = sqlx::query_as("SELECT name FROM players")
        .fetch_all(&*pool)
        .await
        .expect("error while listing player names")
        .into_iter()
        .map(|touple: (String,)| touple.0)
        .collect();
    Ok(result)
}
