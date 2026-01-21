use crate::audio::player::AudioPlayerState;
use crate::database::connection::DbConnection;
use crate::models::track::PlaybackStatus;
use crate::models::track::Track;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;
use rodio::Source;
use rusqlite::params;
use std::fs::File;
use std::io::BufReader;
use tauri::Emitter;
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;
use walkdir::WalkDir;
use std::time::Duration;

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
pub async fn get_tracks(
    folder_path: String,
    db_state: State<'_, DbConnection>,
) -> Result<Vec<Track>, String> {
    let tracks = tokio::task::spawn_blocking(move || {
        // --- Toda la lógica de escaneo se mueve a un hilo bloqueante ---
        // Esto evita que el escaneo de una carpeta grande congele la UI.
        let mut temp_tracks = Vec::new();
        let supported_extensions = ["mp3", "flac", "wav", "m4a", "ogg"];

        for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

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
                            title = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .map(str::to_string);
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
    })
    .await
    .map_err(|e| e.to_string())?;

    // La inserción en la BD se hace después de que el escaneo ha terminado.
    let conn = db_state.inner().0.lock().unwrap();

    conn.execute("DELETE FROM tracks", [])
        .map_err(|e| e.to_string())?;

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
        ])
        .map_err(|e| e.to_string())?;
    }

    println!("{} canciones insertadas en la base de datos.", tracks.len());
    Ok(tracks)
}

#[tauri::command]
pub fn play_track(path: String, audio_state: State<AudioPlayerState>, app_handle: AppHandle) {
    let sink_arc = audio_state.inner().sink.clone();

    {
    let sink = sink_arc.lock().unwrap();

    if !sink.empty() {
        sink.stop();
        sink.clear();
    }

    if let Ok(file) = File::open(&path) {
        if let Ok(source) = rodio::Decoder::new(BufReader::new(file)) {
            let total_duration = source.total_duration().unwrap_or_default();

            sink.append(source);
            sink.play();
            println!("Reproduciendo: {}, Duración: {:?}", path, total_duration);

            app_handle
                .emit(
                    "playback-status-update",
                    PlaybackStatus {
                        position_secs: 0.0,
                        duration_secs: total_duration.as_secs_f64(),
                        is_playing: true,
                    },
                )
                .unwrap();
        } else {
            println!("Error al decodificar: {}", &path);
        }
    } else {
        println!("Error al abrir: {}", &path);
    }
    }

    let monitor_thread_sink = sink_arc;
    let monitor_thread_handle = app_handle;

    std::thread::spawn(move || {
        println!("[Monitor] Hilo de seguimiento iniciado.");
        
        loop {
            let sink = monitor_thread_sink.lock().unwrap();

            let is_empty = sink.empty();
            
            drop(sink);

            if is_empty {
                println!("[Monitor] La canción ha terminado.");
                
                monitor_thread_handle.emit("playback-ended", ()).unwrap();
                
                break;
            }
            
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        
        println!("[Monitor] Hilo de seguimiento terminado.");
    });
    
}

#[tauri::command]
pub fn pause_track(audio_state: State<AudioPlayerState>, app_handle: AppHandle) {
    let sink = audio_state.inner().sink.lock().unwrap();

    if !sink.is_paused() {
        sink.pause();

        app_handle
            .emit(
                "playback-status-update",
                PlaybackStatus {
                    position_secs: 0.0, 
                    duration_secs: 0.0,
                    is_playing: false,
                },
            )
            .unwrap();
    }
}

#[tauri::command]
pub fn resume_track(audio_state: State<AudioPlayerState>, app_handle: AppHandle) {
    let sink = audio_state.inner().sink.lock().unwrap();
    if sink.is_paused() {
        sink.play();

        app_handle.emit(
            "playback-status-update",
            PlaybackStatus {
                position_secs: 0.0, 
                duration_secs: 0.0,
                is_playing: true,
            }
        ).unwrap();
    }
}

#[tauri::command]
pub fn fetch_tracks(db_state: State<'_, DbConnection>) -> Result<Vec<Track>, String> {
    println!("Intentando leer canciones desde la base de datos...");
    let conn = db_state.inner().0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT path, title, artist, album, duration_secs FROM tracks")
        .map_err(|e| e.to_string())?;

    let tracks_iter = stmt
        .query_map([], |row| {
            Ok(Track {
                path: row.get(0)?,
                title: row.get(1)?,
                artist: row.get(2)?,
                album: row.get(3)?,
                duration_secs: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let tracks: Vec<Track> = tracks_iter.filter_map(Result::ok).collect();
    println!(
        "Se encontraron {} canciones en la base de datos.",
        tracks.len()
    );
    Ok(tracks)
}

#[tauri::command]
pub fn set_volume(volume: f32, audio_state: State<AudioPlayerState>) {
    // Aseguramos que el valor esté en el rango correcto (0.0 a 1.0).
    let clamped_volume = volume.clamp(0.0, 1.0);

    let sink = audio_state.inner().sink.lock().unwrap();
    sink.set_volume(clamped_volume);
}

#[tauri::command]
pub fn seek_track(position_secs: f64, audio_state: State<AudioPlayerState>) {
    let sink = audio_state.inner().sink.lock().unwrap();
    
    let seek_time = Duration::from_secs_f64(position_secs);

    if sink.try_seek(seek_time).is_ok() {
        println!("Seek exitoso a: {:?}", seek_time);
    } else {
        println!("Fallo en el seek a: {:?}", seek_time);
    }
}

#[tauri::command]
pub async fn open_add_files_dialog<R: Runtime>(app: AppHandle<R>) -> Option<Vec<String>> {
    let (tx, rx) = oneshot::channel::<Option<Vec<String>>>();

    // Usamos `pick_files`, que es la función que encontraste para selección múltiple.
    app.dialog().file().pick_files(move |file_paths| {
        let result = if let Some(paths) = file_paths {
            // Convertimos el `Vec<FilePath>` a un `Vec<String>`.
            let string_paths = paths.into_iter().map(|p| p.to_string()).collect();
            Some(string_paths)
        } else {
            // El usuario canceló.
            None
        };
        tx.send(result).ok();
    });

    rx.await.unwrap_or(None)
}

#[tauri::command]
pub async fn add_tracks_to_library(paths: Vec<String>, db_state: State<'_, DbConnection>) -> Result<(), String> {
    println!("Añadiendo {} canciones a la biblioteca...", paths.len());
    let conn = db_state.inner().0.lock().unwrap();

    let mut stmt = conn.prepare(
        "INSERT OR IGNORE INTO tracks (path, title, artist, album, duration_secs) VALUES (?1, ?2, ?3, ?4, ?5)"
    ).map_err(|e| e.to_string())?;

    for path_str in paths {
        let path = std::path::Path::new(&path_str);
        
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

            stmt.execute(params![
                path.to_string_lossy().to_string(),
                title,
                artist,
                album,
                Some(properties.duration().as_secs()),
            ]).map_err(|e| e.to_string())?;
        }
    }
    
    println!("Proceso de adición completado.");
    Ok(())
}


