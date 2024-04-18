// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use async_std::sync::Mutex;
use sqlx::{Pool, Sqlite};
use structs::game::Game;
use commands::*;
use vector_util::Vector;

pub mod commands;
pub mod vector_util;
pub mod enums;
pub mod structs;
pub mod db;
const NUMBER_OF_PLAYERS: usize = 4;
const NUMBER_OF_WALLS_PER_PLAYER: i16 = 10;
const BOARD_SIZE: i16 = 9;
const DB_URL: &str = "sqlite://sqlite.db";

pub struct GameState {
    pub game: Mutex<Option<Game>>,
    pub current_possible_moves: Mutex<Option<Vec<Vector>>>,
    pub db_pool: Mutex<Pool<Sqlite>> 
}

#[async_std::main]
async fn main() {
    db::init().await;
    let db_pool = sqlx::sqlite::SqlitePool::connect(DB_URL).await.expect("unable to connect");
    tauri::Builder::default()
        .manage(GameState { game: Mutex::new(None), current_possible_moves: Mutex::new(None), db_pool: Mutex::new(db_pool) })
        .invoke_handler(tauri::generate_handler![start_game, get_player_names, check_wall, move_pawn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
    #[test]
    fn first_test(){
        assert!(true);
    }
}
