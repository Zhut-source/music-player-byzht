use crate::audio::player::AudioPlayerState;
use crate::models::track::Track;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;
use std::fs::File;
use std::io::BufReader;
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;
use walkdir::WalkDir;

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
pub async fn get_tracks(folder_path: String) -> Vec<Track> {
    
    let mut tracks = Vec::new();
    let supported_extensions = ["mp3", "flac", "wav", "m4a", "ogg"];

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let extension = match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.to_lowercase(),
            None => continue,
        };

        if supported_extensions.contains(&extension.as_str()) {
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
                    title = path.file_stem()
                        .and_then(|s| s.to_str())
                        .map(str::to_string);
                }

                tracks.push(Track {
                    path: path.to_string_lossy().to_string(),
                    title,
                    artist,
                    album,
                    duration_secs: Some(properties.duration().as_secs()),
                });
            } else {
                println!("Error al leer metadatos de: {:?}", path);
            }
        }
    }

    println!("Escaneo completado. Se encontraron {} canciones.", tracks.len());
    tracks
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