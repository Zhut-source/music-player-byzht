// Archivo: src/lib.rs

mod commands;
mod models;
mod audio;
mod database;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    
    std::mem::forget(_stream);


    tauri::Builder::default()

        .setup(|app| {
            let conn = database::connection::init_database(&app.handle());
            app.manage(database::connection::DbConnection(std::sync::Mutex::new(conn)));
            Ok(())
        })
        .manage(audio::player::AudioPlayerState {
            sink: std::sync::Mutex::new(sink),
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio_commands::select_folder, 
            commands::audio_commands::get_tracks,
            commands::audio_commands::fetch_tracks,
            commands::audio_commands::play_track,
            commands::audio_commands::pause_track,
            commands::audio_commands::resume_track,
            commands::audio_commands::set_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}