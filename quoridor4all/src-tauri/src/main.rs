// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;

pub mod commands;
pub mod models;
pub mod touple_util;
pub mod enums;
pub mod structs;
const NUMBER_OF_PLAYERS: usize = 4;

struct Counter {
    count: Mutex<i32>,
}

fn main() {
  tauri::Builder::default()
    .manage(Counter { count: Mutex::new(0) })
    .invoke_handler(tauri::generate_handler![inc, dec])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn inc(state: State<Counter>) -> i32{
    let mut counter = state.count.lock().unwrap();
    *counter = *counter + 1;
    *counter
}

#[tauri::command]
fn dec(state: State<Counter>) -> i32{
    let mut counter = state.count.lock().unwrap();
    *counter = *counter - 1;
    *counter
}

#[cfg(test)]
mod test {
    #[test]
    fn first_test(){
        assert!(true);
    }
}
