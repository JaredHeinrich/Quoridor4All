use std::ops::DerefMut;

use tauri::State;

use crate::{db::models::DbPlayer, structs::{game::{Game, Player}, history::Move, wall::Wall}, vector_util::Vector, GameState, BOARD_SIZE, NUMBER_OF_PLAYERS, NUMBER_OF_WALLS_PER_PLAYER};

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

#[tauri::command]
pub async fn get_possible_moves<'a>(state: State<'a, GameState>) -> Result<Vec<Vector>, String> {
    let mut moves_lock = state.current_possible_moves.lock().await;
    let game_lock = state.game.lock().await;
    let result = match moves_lock.as_ref() {
        Some(m) => m.to_vec(),
        None => {
            let game = match game_lock.as_ref() {
                Some(g) => g.clone(),
                None => return Err("no game running".to_string()),
            };
            let res = game.get_valid_next_positions();
            *moves_lock = Some(res.clone());
            res.to_vec()
        },
    };
    Ok(result)
}

#[tauri::command]
pub async fn move_pawn<'a>(state: State<'a, GameState>, new_position: Vector) -> Result<Vector, String> {
    let mut moves_lock = state.current_possible_moves.lock().await;
    let mut game_lock = state.game.lock().await;
    let result: Result<Vector, String> = match moves_lock.as_ref() {
        //if there are buffered positions use them
        Some(allowed_moves) => {
            match game_lock.deref_mut() {
                Some(g) => {
                    g.move_current_pawn(new_position, allowed_moves)
                },
                None => return Err("no game running".to_string()),
            }
        },
        //if there are no buffered next positions calculate them
        None => {
            match game_lock.deref_mut() {
                Some(g) => {
                    let allowed_moves = g.get_valid_next_positions();
                    g.move_current_pawn(new_position, &allowed_moves)
                },
                None => return Err("no game running".to_string()),
            }
        }
    };
    match result {
        Ok(_) => {
            //wenn der move erfolgreich war die gepufferten moves entfernen.
            *moves_lock = None;
        },
        _ => {},
    }
    result
}

#[tauri::command]
pub async fn check_wall<'a>(state: State<'a, GameState>, wall: Wall) -> Result<bool, String> {
    let game_lock = state.game.lock().await;
    let res: bool = match game_lock.as_ref() {
        Some(g) => g.is_wall_valid(&wall),
        None => return Err("no game running".to_string()),
    };
    Ok(res)
}

#[tauri::command]
pub async fn place_wall<'a>(state: State<'a, GameState>, wall: Wall) -> Result<(), String> {
    let mut game_lock = state.game.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    let result = match game_lock.as_mut() {
        Some(g) => g.place_wall(wall),
        None => return Err("no game running".to_string()),
    };
    match result {
        Ok(_) => {
            //wenn der move erfolgreich war die gepufferten moves entfernen.
            *moves_lock = None;
        },
        _ => {},
    };
    result
}
#[tauri::command]
pub async fn undo_last_move<'a>(state: State<'a, GameState>) -> Result<(Vector, bool), String> {
    let mut game_lock = state.game.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    let result = match game_lock.as_mut() {
        Some(g) => g.undo_last_move(),
        None => return Err("no game running".to_string()),
    };
    *moves_lock = None;
    result
}

#[tauri::command]
pub async fn get_top_players<'a>(state: State<'a, GameState>) -> Result<Vec<DbPlayer>, String> {
    let pool = state.db_pool.lock().await;
    let result: Result<Vec<DbPlayer>, String>= sqlx::query_as(" SELECT name, wins FROM players ORDER BY wins DESC LIMIT 3 ")
        .fetch_all(&*pool).await.map_err(|err| {err.to_string()});
    result
}

#[tauri::command]
pub async fn cancel_game<'a>(state: State<'a, GameState>) -> Result<(), String> {
    let mut game_lock = state.game.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    *game_lock = None;
    *moves_lock = None;
    Ok(())
}
