use std::ops::DerefMut;

use tauri::State;

use crate::{db::models::DbPlayer, structs::{game::{Game, Player}, wall::Wall}, vector_util::Vector, GameState, BOARD_SIZE, NUMBER_OF_PLAYERS, NUMBER_OF_WALLS_PER_PLAYER};

//Command, um  ein neues Spiel zu starten.
#[tauri::command]
pub async fn start_game<'a>(players: [Player; NUMBER_OF_PLAYERS], state: State<'a, GameState>) -> Result<(), String> {
    let pool = state.db_pool.lock().await;
    // die 4 Spieler in die Datenbank eintragen, falls es sie noch nicht gibt.
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
    //Das game im State initialisieren.
    let mut game = state.game.lock().await;
    *game = Some(Game::new(BOARD_SIZE, NUMBER_OF_WALLS_PER_PLAYER, players));
    Ok(())
}

//Gibt eine Liste aller Spielernamen zurück. (wird noch nicht verwendet)
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

//Gibt alle möglichen moves für den aktuellen Pawn zurück.
#[tauri::command]
pub async fn get_possible_moves<'a>(state: State<'a, GameState>) -> Result<Vec<Vector>, String> {
    let mut moves_lock = state.current_possible_moves.lock().await;
    let game_lock = state.game.lock().await;
    let result = match moves_lock.as_ref() {
        //wenn bereits züge gepuffert wurden, dann werden diese zurückgegeben.
        Some(m) => m.to_vec(),
        None => {
            let game = match game_lock.as_ref() {
                Some(g) => g.clone(),
                None => return Err("no game running".to_string()),
            };
            let res = game.get_valid_next_positions();
            //die Züge im puffer spiechern.
            *moves_lock = Some(res.clone());
            res.to_vec()
        },
    };
    //Züge zurückgeben.
    Ok(result)
}

#[tauri::command]
pub async fn move_pawn<'a>(state: State<'a, GameState>, new_position: Vector) -> Result<(Vector, bool), String> {
    let pool = state.db_pool.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    let mut game_lock = state.game.lock().await;
    let result: Result<(Vector, bool, String), String> = match moves_lock.as_ref() {
        //wenn es gepufferte einträge gibt werden diese genutzt um zu überprüfen, ob der move valid ist.
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
        Ok(r) => {
            //wenn der move erfolgreich war die gepufferten moves entfernen.
            *moves_lock = None;
            // wenn der spieler gewonnen hat wird das spiel gelöscht
            if r.1 {
                *game_lock = None;
                //Die Wins des gewinners werden um 1 erhöht.
                let _res = sqlx::query("
                                       UPDATE players
                                       SET wins = wins + 1
                                       WHERE name = $1
                                          ")
                    .bind(&r.2)
                    .execute(&*pool)
                    .await;
            }
            Ok((r.0,r.1))
        },
        Err(e) => Err(e),
    }
}

//gibt zurück, ob die Wall plaziert werden darf.
#[tauri::command]
pub async fn check_wall<'a>(state: State<'a, GameState>, wall: Wall) -> Result<bool, String> {
    let game_lock = state.game.lock().await;
    let res: bool = match game_lock.as_ref() {
        Some(g) => g.is_wall_valid(&wall),
        None => return Err("no game running".to_string()),
    };
    Ok(res)
}

//plaziert eine Wand wenn es sie plaziert werden darf.
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

//den letzten Zug rückgängig machen.
#[tauri::command]
pub async fn undo_last_move<'a>(state: State<'a, GameState>) -> Result<(Vector, bool), String> {
    let mut game_lock = state.game.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    let result = match game_lock.as_mut() {
        Some(g) => g.undo_last_move(),
        None => return Err("no game running".to_string()),
    };
    //setzt die gepufferten Züge zurück.
    *moves_lock = None;
    result
}

//gibt die 3 spieler mit den meisten Siegen zurück.
#[tauri::command]
pub async fn get_top_players<'a>(state: State<'a, GameState>) -> Result<Vec<DbPlayer>, String> {
    let pool = state.db_pool.lock().await;
    let result: Result<Vec<DbPlayer>, String>= sqlx::query_as(" SELECT name, wins FROM players ORDER BY wins DESC LIMIT 3 ")
        .fetch_all(&*pool).await.map_err(|err| {err.to_string()});
    result
}

//entfernt das Spiel aus dem State.
#[tauri::command]
pub async fn cancel_game<'a>(state: State<'a, GameState>) -> Result<(), String> {
    let mut game_lock = state.game.lock().await;
    let mut moves_lock = state.current_possible_moves.lock().await;
    *game_lock = None;
    *moves_lock = None;
    Ok(())
}
