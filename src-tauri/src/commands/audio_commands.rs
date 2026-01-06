use crate::database::connection::DbConnection;
use crate::models::track::Track;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;
use rusqlite::params;
use std::fs::File;
use std::io::BufReader;
use tauri::{AppHandle, Runtime, State}; // <-- CORREGIDO: `State` importado una sola vez
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;
use walkdir::WalkDir;
use crate::audio::player::AudioPlayerState;

#[tauri::command]
pub async fn select_folder<R: Runtime>(app: AppHandle<R>) -> Option<String> {
    let (tx, rx) = oneshot::channel::<Option<String>>();

    app.dialog().file().pick_folder(move |folder_path| {
        let result = folder_path.map(|path| path.to_string());
        tx.send(result).ok();
    });

    rx.await.unwrap_or(None)
}

#[tauri::command]
pub async fn get_tracks(folder_path: String, db_state: State<'_, DbConnection>) -> Result<Vec<Track>, String> {
    let tracks = tokio::task::spawn_blocking(move || {
        // --- Toda la lógica de escaneo se mueve a un hilo bloqueante ---
        // Esto evita que el escaneo de una carpeta grande congele la UI.
        let mut temp_tracks = Vec::new();
        let supported_extensions = ["mp3", "flac", "wav", "m4a", "ogg"];

        for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() { continue; }

            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                if supported_extensions.contains(&extension.to_lowercase().as_str()) {
                    if let Ok(tagged_file) = lofty::read_from_path(path) {
                        let properties = tagged_file.properties();
                        let mut title = None;
                        let mut artist = None;
                        let mut album = None;

                        if let Some(tag) = tagged_file.primary_tag() {
                            title = tag.title().map(|s| s.to_string());
                            artist = tag.artist().map(|s| s.to_string());
                            album = tag.album().map(|s| s.to_string());
                        }

                        if title.is_none() {
                            title = path.file_stem().and_then(|s| s.to_str()).map(str::to_string);
                        }

                        temp_tracks.push(Track {
                            path: path.to_string_lossy().to_string(),
                            title,
                            artist,
                            album,
                            duration_secs: Some(properties.duration().as_secs()),
                        });
                    }
                }
            }
        }
        temp_tracks
        // --- Fin del hilo bloqueante ---
    }).await.map_err(|e| e.to_string())?;


    // La inserción en la BD se hace después de que el escaneo ha terminado.
    let conn = db_state.inner().0.lock().unwrap();

    conn.execute("DELETE FROM tracks", []).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO tracks (path, title, artist, album, duration_secs) VALUES (?1, ?2, ?3, ?4, ?5)"
    ).map_err(|e| e.to_string())?;

    for track in &tracks {
        stmt.execute(params![
            &track.path,
            &track.title,
            &track.artist,
            &track.album,
            &track.duration_secs,
        ]).map_err(|e| e.to_string())?;
    }
    
    println!("{} canciones insertadas en la base de datos.", tracks.len());
    Ok(tracks)
}


#[tauri::command]
pub fn play_track(path: String, audio_state: State<AudioPlayerState>) {
    let sink = audio_state.inner().sink.lock().unwrap();
    
    // --- LÓGICA CORREGIDA ---
    
    // 1. Si el Sink no está vacío, lo detenemos y lo limpiamos.
    if !sink.empty() {
        sink.stop();
        // `clear()` elimina todas las canciones en cola, asegurando que esté vacío.
        sink.clear(); 
    }
    
    // 2. Ahora que estamos seguros de que el Sink está limpio, añadimos la nueva canción.
    if let Ok(file) = File::open(path) {
        if let Ok(source) = rodio::Decoder::new(BufReader::new(file)) {
            sink.append(source);
            sink.play();
        } else {
            println!("Error al decodificar el archivo de audio.");
        }
    } else {
        println!("Error al abrir el archivo de audio.");
    }
}

#[tauri::command]
pub fn pause_track(audio_state: State<AudioPlayerState>) {
    let sink = audio_state.inner().sink.lock().unwrap();
    if !sink.is_paused() {
        sink.pause();
    }
}

#[tauri::command]
pub fn resume_track(audio_state: State<AudioPlayerState>) {
    let sink = audio_state.inner().sink.lock().unwrap();
    if sink.is_paused() {
        sink.play();
    }
}

#[tauri::command]
pub fn fetch_tracks(db_state: State<'_, DbConnection>) -> Result<Vec<Track>, String> {
    println!("Intentando leer canciones desde la base de datos...");
    let conn = db_state.inner().0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT path, title, artist, album, duration_secs FROM tracks")
        .map_err(|e| e.to_string())?;

    let tracks_iter = stmt.query_map([], |row| {
        Ok(Track {
            path: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            album: row.get(3)?,
            duration_secs: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

    let tracks: Vec<Track> = tracks_iter.filter_map(Result::ok).collect();
    println!("Se encontraron {} canciones en la base de datos.", tracks.len());
    Ok(tracks)
}