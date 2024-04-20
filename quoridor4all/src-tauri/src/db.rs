use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool };

use crate::DB_URL;

//datenbank wird inital erstellt.
pub async fn init() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    //Datenbank Tabellen werden angelegt.
    create_schema().await;
}

// Create Schema
async fn create_schema() {
    let pool = SqlitePool::connect(DB_URL).await.expect("unable to connect");
    let sql = "
        CREATE TABLE IF NOT EXISTS players 
        (
            name TEXT PRIMARY KEY NOT NULL,
            wins INTEGER NOT NULL
        );
    ";
    
    let query = sqlx::query(&sql);
    let result = query.execute(&pool).await.unwrap();
    println!("Create Schema result: {:?}", result);   
    pool.close().await;
}
pub mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::prelude::FromRow;

    //Represetation eines Datenbank Eintrags
    #[derive(Debug, FromRow, Serialize, Deserialize)]
    pub struct DbPlayer {
        name: String,
        wins: u32
    }
}
