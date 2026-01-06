// Archivo: src/database/connection.rs

use rusqlite::Connection;
use std::{path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Manager};

pub struct DbConnection(pub Mutex<Connection>);

pub fn init_database(app_handle: &AppHandle) -> Connection {
    let db_path: PathBuf;
    if cfg!(debug_assertions) {
        // --- LÓGICA DE RUTA PERSONALIZADA ---
        
        // 1. Obtenemos la ruta a la carpeta `src-tauri`.
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        
        // 2. Navegamos un nivel hacia arriba para llegar a la raíz del proyecto
        //    (a la carpeta que contiene `src-tauri`).
        let project_root = manifest_dir.parent().expect("Failed to get project root");

        // 3. Creamos la carpeta `BD` en la raíz del proyecto.
        let db_folder = project_root.join("BD");
        std::fs::create_dir_all(&db_folder).expect("Failed to create BD directory");
        
        // 4. La ruta final de nuestro archivo de base de datos.
        db_path = db_folder.join("library.db");

    } else {
        // La lógica de producción se queda igual (usa AppData).
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