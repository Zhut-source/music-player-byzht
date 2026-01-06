// Archivo: src/database/connection.rs

use rusqlite::Connection;
use std::{path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Manager};

const DEV_DB_PATH: &str = "G:\\doc importantes\\Proyectos ZHUT\\music-player-v1\\music-player\\BD\\library.db";

pub struct DbConnection(pub Mutex<Connection>);

pub fn init_database(app_handle: &AppHandle) -> Connection {
    let db_path: PathBuf;   

   if cfg!(debug_assertions) {
        // MODO DESARROLLO: Usa la ruta que definiste en la constante.
        db_path = PathBuf::from(DEV_DB_PATH);
        
        // Nos aseguramos de que la carpeta contenedora exista.
        if let Some(parent_dir) = db_path.parent() {
            std::fs::create_dir_all(parent_dir).expect("Failed to create custom DB directory");
        }

    } else {
        let app_data_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
        db_path = app_data_dir.join("library.db");
    }

    println!("Ruta de la base de datos: {:?}", db_path);

    let conn = Connection::open(db_path).expect("Failed to open database connection");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            path          TEXT PRIMARY KEY,
            title         TEXT,
            artist        TEXT,
            album         TEXT,
            duration_secs INTEGER
        )",
        [],
    ).expect("Failed to create tracks table");

    conn
}