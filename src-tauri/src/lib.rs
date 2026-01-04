// Archivo: src-tauri/src/lib.rs

use tauri::{Runtime, AppHandle};
use tauri_plugin_dialog::DialogExt;
// Importamos el canal `oneshot` de Tokio.
use tokio::sync::oneshot;

#[tauri::command]
async fn select_folder<R: Runtime>(app: AppHandle<R>) -> Option<String> {
    // 1. Creamos un canal de un solo uso.
    // tx = transmisor, rx = receptor.
    let (tx, rx) = oneshot::channel::<Option<String>>();

    // 2. Llamamos a `pick_folder` con un callback (una clausura).
    //    Usamos la API exacta que encontraste en la documentación.
    app.dialog().file().pick_folder(move |folder_path| {
        // Esta parte del código se ejecuta DESPUÉS, cuando el usuario cierra el diálogo.

        let result = if let Some(path) = folder_path {
            // Si el usuario eligió una carpeta, la convertimos a String.
            Some(path.to_string())
        } else {
            // Si el usuario canceló, el resultado es None.
            None
        };

        // 3. Enviamos el resultado a través del canal al `async fn` que está esperando.
        //    Usamos .ok() para ignorar un posible error si el receptor ya no está escuchando.
        tx.send(result).ok();
    });

    // 4. Nuestra función async se detiene aquí y espera (`await`) a que el callback
    //    envíe un valor a través del canal.
    //    rx.await devuelve un Result, por lo que usamos .unwrap_or(None) para manejar
    //    el caso de que el canal se cierre sin enviar nada.
    rx.await.unwrap_or(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}