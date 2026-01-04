// Archivo: src-tauri/src/commands/audio_commands.rs
use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;
use walkdir::WalkDir;

use crate::models::track::Track;

#[tauri::command]
pub async fn select_folder<R: Runtime>(app: AppHandle<R>) -> Option<String> {
    // ... (código existente)
    let (tx, rx) = oneshot::channel::<Option<String>>();
    app.dialog().file().pick_folder(move |folder_path| {
        let result = if let Some(path) = folder_path {
            Some(path.to_string())
        } else {
            None
        };
        tx.send(result).ok();
    });
    rx.await.unwrap_or(None)
}

#[tauri::command]
pub async fn get_tracks(folder_path: String) -> Vec<Track> {
    println!("Iniciando escaneo de la carpeta: {}", folder_path);
    let mut tracks = Vec::new();
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        println!("Encontrado: {:?}", entry.path());
    }
    println!("Escaneo completado.");
    tracks
}