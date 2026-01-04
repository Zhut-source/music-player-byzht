
mod commands;
mod models;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio_commands::select_folder, 
            commands::audio_commands::get_tracks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}